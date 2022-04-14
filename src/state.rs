use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Eq, Debug, PartialEq, Clone)]
pub struct ProjectInfo {
    pub bank: Pubkey,
    pub owner: Pubkey,
    pub milestone: u64,
    pub raised: u64,
    pub name: String,
}

impl ProjectInfo {
    pub fn size(&self) -> Option<u64> {
        std::mem::size_of_val::<Self>(self).try_into().ok()
    }
}
