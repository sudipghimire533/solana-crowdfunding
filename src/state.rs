use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Eq, Debug, PartialEq, Clone)]
pub struct ProjectInfo {
    pub target: u64,
    pub bank: Pubkey,
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
}
