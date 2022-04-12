use crate::processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

fn main(program_id: &Pubkey, account_info: &[AccountInfo], instructions: &[u8]) -> ProgramResult {
    processor::Processor::process(program_id, account_info, instructions)
}

entrypoint!(main);
