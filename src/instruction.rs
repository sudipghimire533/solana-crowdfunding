use crate::error::CrowdError;
use crate::state::ProjectInfo;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{msg, pubkey::Pubkey};

#[derive(Debug)]
#[cfg_attr(test, derive(BorshSerialize))]
pub enum Instruction {
    Create { params: CreateParams },

    Compliment { params: ComplimentParams },
}
#[derive(Debug)]
#[cfg_attr(test, derive(BorshSerialize))]
pub struct CreateParams {
    pub target: u64,
    pub name: String,
    pub project_bump: u8,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone)]
pub struct ComplimentParams {
    amount: u64,
}

impl Instruction {
    pub fn unpack(instruction_bytes: &[u8]) -> Result<Self, CrowdError> {
        use CrowdError::InvalidInstruction;

        let (index, data) = instruction_bytes.split_first().ok_or(InvalidInstruction)?;
        let instruction: Instruction;

        match *index {
            0 => {
                let (mut target_bytes, rest) = data.split_at(8);
                let target = <u64 as BorshDeserialize>::deserialize(&mut target_bytes)
                    .map_err(|_| InvalidInstruction)?;

                let (mut name_length_bytes, rest) = rest.split_at(4);
                let name_length = <u32 as BorshDeserialize>::deserialize(&mut name_length_bytes)
                    .map_err(|_| InvalidInstruction)? as usize;

                let (name_bytes, rest) = rest.split_at(name_length);
                let name =
                    String::from_utf8(name_bytes.to_vec()).map_err(|_| InvalidInstruction)?;

                let (project_bump_bytes, _rest) = rest.split_at(1);
                let project_bump = *project_bump_bytes.first().ok_or(InvalidInstruction)?;

                instruction = Instruction::Create {
                    params: CreateParams {
                        name,
                        target,
                        project_bump,
                    },
                };
            }

            1 => {
                let (mut amount_bytes, _rest) = data.split_at(8);
                let amount = <u64 as BorshDeserialize>::deserialize(&mut amount_bytes)
                    .map_err(|_| InvalidInstruction)?;

                instruction = Instruction::Compliment {
                    params: ComplimentParams { amount },
                }
            }
            _ => {
                return Err(InvalidInstruction);
            }
        }

        Ok(instruction)
    }
}
