#![allow(unused_imports)]

use crate::instruction::{self, Instruction};
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
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
