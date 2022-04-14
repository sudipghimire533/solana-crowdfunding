#![allow(unused_imports)]

use crate::error::{self, CrowdError};
use solana_program::{self, msg, pubkey::Pubkey, rent};

pub const RENT: rent::Rent = solana_program::rent::Rent {
    lamports_per_byte_year: rent::DEFAULT_LAMPORTS_PER_BYTE_YEAR,
    exemption_threshold: rent::DEFAULT_EXEMPTION_THRESHOLD,
    burn_percent: rent::DEFAULT_BURN_PERCENT,
};

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
