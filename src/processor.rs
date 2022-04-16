#![allow(unused_imports)]

use crate::error::{self, CrowdError};
use crate::instruction::{self, Instruction};
use crate::state::{self, ProjectInfo};
use crate::utils;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    self,
    account_info::{self, next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg, program,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};
pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        account_info: &[AccountInfo],
        instructions: &[u8],
    ) -> ProgramResult {
        let instruction = Instruction::unpack(instructions)?;
        msg!("Instruction is: {:?}", instruction);

        match instruction {
            Instruction::Create { params } => {
                Self::create_project(params, program_id, account_info)
            }

            Instruction::Compliment { params } => {
                Self::compliment_project(params, program_id, account_info)
            }

            Instruction::Withdraw => Self::withdraw_funds(program_id, account_info),
        }
    }

    fn create_project(
        params: instruction::CreateParams,
        program_id: &Pubkey,
        account_info: &[AccountInfo],
    ) -> ProgramResult {
        let accounts_iter = &mut account_info.iter();

        // Collect required accounts.
        // System account to create other accounts
        // creator is who is owner of this program
        // bank is pda from creator
        // project is pda from creator
        let system_account = next_account_info(accounts_iter)?;
        let creator = next_account_info(accounts_iter)?;
        let bank = next_account_info(accounts_iter)?;
        let project = next_account_info(accounts_iter)?;

        // refer public key for
        // Creator is who is creating the peoject so must be signed
        // bank address is unsigned as it is pda
        // project_address is unsigned as it is pda
        let creator_address = creator.signer_key().ok_or(CrowdError::IllegalCreator)?;
        let bank_address = bank.unsigned_key();
        let project_address = project.unsigned_key();

        // Bank is derived from creater publick address and name of project
        // so name must be unique for a single creator
        let bank_seed = &[creator.unsigned_key().as_ref(), params.name.as_bytes()][..];
        let bank_bump = utils::verify_pda(program_id, bank_address, bank_seed)
            .map_err(|_| CrowdError::UnexpectedBankAddress)?;

        // Address seed is bank seed combined with bank bump
        let bank_bump_slice = &[bank_bump][..];
        let project_seed = &[bank_address.as_ref(), bank_bump_slice][..];
        let project_bump = utils::verify_pda(program_id, project_address, project_seed)
            .map_err(|_| CrowdError::UnexpectedProjectAddress)?;

        // Final bump be as expected as passed in data
        if project_bump != params.project_bump {
            Err(CrowdError::UnexpectedBump)?;
        }

        // Create a project details
        let project_info = ProjectInfo {
            bank: bank_address.clone(),
            owner: creator_address.clone(),
            milestone: params.target,
            raised: 0,
            name: params.name.clone(),
            withdrawn: false,
        };

        // Create a bank account with 0 space
        // and enough rent exemption
        {
            let create_bank_instruction = {
                let space = 0u64;
                let lamports = utils::RENT.minimum_balance(space as usize);
                system_instruction::create_account(
                    creator_address,
                    bank_address,
                    lamports,
                    space,
                    system_account.unsigned_key(),
                )
            };

            program::invoke_signed(
                &create_bank_instruction,
                &[creator.clone(), bank.clone()],
                &[&[bank_seed[0], bank_seed[1], &[bank_bump]]],
            )
            .map_err(|e| {
                if e == ProgramError::AccountAlreadyInitialized {
                    CrowdError::BankAddresCollision.into()
                } else {
                    e
                }
            })?;
        }

        // Create project address with size of project info
        // and with minimum lamports to store it
        {
            let create_project_instruction = {
                let space = project_info.size().ok_or(CrowdError::LargeProjectInfo)?;
                let lamports = utils::RENT.minimum_balance(space.try_into().unwrap());
                system_instruction::create_account(
                    &creator.key,
                    &project.key,
                    lamports,
                    space,
                    program_id,
                )
            };
            program::invoke_signed(
                &create_project_instruction,
                &[project.clone(), creator.clone()],
                &[&[project_seed[0], project_seed[1], &[project_bump]]],
            )
            .map_err(|e| {
                if e == ProgramError::AccountAlreadyInitialized {
                    CrowdError::ProjectAddressCollision.into()
                } else {
                    e
                }
            })?;
        }

        // Store project info
        <ProjectInfo as BorshSerialize>::serialize(
            &project_info,
            &mut &mut project.try_borrow_mut_data()?[..],
        )?;

        Ok(())
    }

    fn compliment_project(
        params: instruction::ComplimentParams,
        program_id: &Pubkey,
        account_info: &[AccountInfo],
    ) -> ProgramResult {
        let accounts_iter = &mut account_info.iter();

        // Collect required accounts.
        // System account to create other accounts
        // creator is who is owner of this program
        // bank is pda from creator
        // project is pda from creator
        let system_account = next_account_info(accounts_iter)?;
        let creditor = next_account_info(accounts_iter)?;
        let project = next_account_info(accounts_iter)?;
        let bank = next_account_info(accounts_iter)?;

        let creditor_address = creditor.signer_key().ok_or(CrowdError::IllegalDonator)?;
        let mut project_info = ProjectInfo::deserialize(&mut &**project.try_borrow_data()?)
            .map_err(|_| CrowdError::CorruptedProjectData)?;

        if project_info.withdrawn {
            Err(CrowdError::InactiveProject)?;
        }
        if bank.unsigned_key().ne(&project_info.bank) {
            Err(CrowdError::BankAddressMismatch)?;
        }
        if project.owner.ne(program_id) {
            Err(CrowdError::IllegalProjectAddressOwner)?;
        }

        // Do actual lamports transfer
        let donate_instruction =
            system_instruction::transfer(creditor_address, &project_info.bank, params.amount);
        program::invoke(
            &donate_instruction,
            &[system_account.clone(), creditor.clone(), bank.clone()],
        )?;

        // Update info in project address itself
        project_info.raised += params.amount;

        // Write updated data in project
        <ProjectInfo as BorshSerialize>::serialize(
            &project_info,
            &mut &mut project.try_borrow_mut_data()?[..],
        )?;

        Ok(())
    }

    pub fn withdraw_funds(program_id: &Pubkey, account_info: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut account_info.iter();

        let system_account = next_account_info(accounts_iter)?;
        let withdrawer = next_account_info(accounts_iter)?;
        let project = next_account_info(accounts_iter)?;
        let bank = next_account_info(accounts_iter)?;

        let withdrawer_address = withdrawer
            .signer_key()
            .ok_or(CrowdError::IllegalWithdrawer)?;

        let mut project_info = ProjectInfo::deserialize(&mut &**project.try_borrow_data()?)
            .map_err(|_| CrowdError::CorruptedProjectData)?;

        msg!("{:#?}", project_info);

        if bank.unsigned_key().ne(&project_info.bank) {
            Err(CrowdError::UnexpectedBankAddress)?;
        }
        if withdrawer_address.ne(&project_info.owner) {
            Err(CrowdError::IllegalWithdrawer)?;
        }
        if project_info.raised < project_info.milestone {
            Err(CrowdError::UnfulfilledMilestone)?;
        }
        if project_info.withdrawn {
            Err(CrowdError::InactiveProject)?;
        }

        let bank_seed = &[withdrawer_address.as_ref(), project_info.name.as_bytes()][..];
        let bank_bump = utils::verify_pda(program_id, bank.unsigned_key(), bank_seed)
            .map_err(|_| CrowdError::UnexpectedBankAddress)?;

        // Transfer from project bank to withdrawer address
        {
            let transfer_instruction = system_instruction::transfer(
                &project_info.bank,
                &project_info.owner,
                bank.lamports(),
            );
            program::invoke_signed(
                &transfer_instruction,
                &[system_account.clone(), bank.clone(), withdrawer.clone()],
                &[&[bank_seed[0], bank_seed[1], &[bank_bump]]],
            )?;
        }

        project_info.withdrawn = true;

        // Write updated data in project
        <ProjectInfo as BorshSerialize>::serialize(
            &project_info,
            &mut &mut project.try_borrow_mut_data()?[..],
        )?;

        Ok(())
    }
}
