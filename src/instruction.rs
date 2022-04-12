use crate::error::CrowdError;
use crate::state::ProjectInfo;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub enum Instruction {
    Create { params: CreateParams },

    Compliment { params: ComplimentParams },
}
#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone)]
pub struct CreateParams {
    project: ProjectInfo,
    address: Pubkey,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone)]
pub struct ComplimentParams {
    address: Pubkey,
    amount: u64,
}

impl Instruction {
    pub fn unpack(instruction_bytes: &[u8]) -> Result<Self, CrowdError> {
        use CrowdError::InvalidInstruction;

        let (index, mut data) = instruction_bytes.split_first().ok_or(InvalidInstruction)?;
        let instruction: Instruction;

        match *index {
            1 => {
                let params = <CreateParams as BorshDeserialize>::deserialize(&mut data)
                    .map_err(|_| CrowdError::InvalidInstruction)?;
                instruction = Instruction::Create { params }
            }

            2 => {
                let params = <ComplimentParams as BorshDeserialize>::deserialize(&mut data)
                    .map_err(|_| CrowdError::InvalidInstruction)?;
                instruction = Instruction::Compliment { params }
            }

            _ => {
                return Err(InvalidInstruction);
            }
        }

        Ok(instruction)
    }
}
