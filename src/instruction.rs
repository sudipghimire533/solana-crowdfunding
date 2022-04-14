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
    pub amount: u64,
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

#[test]
fn decode_program_info() {
    let bytes: &'static [u8] = &[
        239, 235, 154, 51, 49, 12, 176, 239, 99, 184, 111, 48, 121, 138, 50, 89, 245, 68, 194, 192,
        163, 104, 135, 15, 185, 191, 147, 174, 159, 175, 158, 80, 174, 215, 37, 55, 92, 154, 200,
        27, 251, 169, 125, 127, 173, 229, 243, 235, 233, 122, 253, 252, 114, 223, 125, 234, 174,
        172, 80, 149, 20, 76, 136, 200, 0, 228, 11, 84, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0,
        0, 0, 99, 114, 111, 119, 100, 102, 117, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let (mut account_bytes, rest) = bytes.split_at(32);
    let account = <Pubkey as BorshDeserialize>::deserialize(&mut account_bytes).unwrap();
    println!("Account is {:?}", account);

    let (mut account_bytes, rest) = rest.split_at(32);
    let account = <Pubkey as BorshDeserialize>::deserialize(&mut account_bytes).unwrap();
    println!("Account is {:?}", account);

    let (mut lamports_bytes, rest) = rest.split_at(8);
    let lamports = <u64 as BorshDeserialize>::deserialize(&mut lamports_bytes).unwrap();
    println!("Milestone is {}", lamports);

    let (mut lamports_bytes, rest) = rest.split_at(8);
    let lamports = <u64 as BorshDeserialize>::deserialize(&mut lamports_bytes).unwrap();
    println!("Raised is {}", lamports);

    // let (mut size_bytes, rest) = rest.split_at(4);
    // let size_bytes = <u32 as BorshDeserialize>::deserialize(&mut size_bytes).unwrap();
    // println!("Name length is {}", size_bytes);

    let mut name_bytes = &rest[..];
    let name = <String as BorshDeserialize>::deserialize(&mut name_bytes).unwrap();
    println!("Name is {}", name);
}
