#![allow(unused_imports)]

use crate::error::{self, CrowdError};
use solana_program::{self, msg, pubkey::Pubkey};

pub fn verify_pda(
    program_id: &Pubkey,
    expected_address: &Pubkey,
    seeds: &[&[u8]],
) -> Result<u8, Option<Pubkey>> {
    let mut calculated_address: Option<Pubkey> = None;

    Pubkey::try_find_program_address(seeds, program_id)
        .map(|(address, bump)| {
            calculated_address = Some(address);

            if address == *expected_address {
                Some(bump)
            } else {
                None
            }
        })
        .flatten()
        .ok_or(calculated_address)
}

pub fn verify_bank_address<'a>(
    program_id: &Pubkey,
    bank_address: &'a Pubkey,
    project_name: &str,
    project_owner: &Pubkey,
) -> Result<u8, CrowdError> {
    let bank_seed = &[project_owner.as_ref(), project_name.as_bytes()][..];

    verify_pda(program_id, bank_address, bank_seed).map_err(|calculated| {
        msg!(
            "[Verify bank address] Expected {:?} & Calculated {:?}",
            bank_address,
            calculated
        );
        CrowdError::UnexpectedBankAddress
    })
}

pub fn verify_project_address<'a>(
    program_id: &Pubkey,
    project_address: &'a Pubkey,
    (bank_address, bank_bump): (&Pubkey, u8),
) -> Result<u8, CrowdError> {
    let bank_bump = [bank_bump];
    let project_seed = &[bank_address.as_ref(), &bank_bump][..];

    verify_pda(program_id, project_address, project_seed).map_err(|calculated| {
        msg!(
            "[Verify project address] Expected {:?} & Calculated {:?}",
            project_address,
            calculated
        );

        CrowdError::UnexpectedProjectAddress
    })
}
