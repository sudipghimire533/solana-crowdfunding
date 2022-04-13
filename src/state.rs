use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Eq, Debug, PartialEq, Clone)]
pub struct ProjectInfo {
    pub target: u64,
    pub bank: Pubkey,
    pub owner: Pubkey,
    pub name: String,
}

#[cfg(test)]
impl Default for ProjectInfo {
    fn default() -> Self {
        ProjectInfo {
            target: 10,
            bank: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            name: "Some name".to_string(),
        }
    }
}
