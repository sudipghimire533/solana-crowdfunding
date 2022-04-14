#![allow(unused_imports)]

use crate::error::{self, CrowdError};
use crate::instruction::{self, Instruction};
use crate::utils;
use solana_program::{
    self,
    account_info::{self, next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
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
        }
    }

    fn create_project(
        params: instruction::CreateParams,
        program_id: &Pubkey,
        account_info: &[AccountInfo],
    ) -> ProgramResult {
        let accounts_iter = &mut account_info.iter();

        let system_account = next_account_info(accounts_iter)?;
        let creator = next_account_info(accounts_iter)?;
        let bank = next_account_info(accounts_iter)?;
        let address = next_account_info(accounts_iter)?;

        let bank_bump = utils::verify_bank_address(
            program_id,
            bank.unsigned_key(),
            &params.name,
            creator.signer_key().ok_or(CrowdError::IllegalCreator)?,
        )?;
        let project_bump = utils::verify_project_address(
            program_id,
            address.unsigned_key(),
            (&bank.key, bank_bump),
        )?;

        if project_bump != params.project_bump {
            Err(CrowdError::UnexpectedBump)?;
        }

        todo!();

        Ok(())
    }

    fn compliment_project(
        params: instruction::ComplimentParams,
        program_id: &Pubkey,
        account_info: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }
}
