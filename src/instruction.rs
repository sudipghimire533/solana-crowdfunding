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
#[derive(Debug, BorshDeserialize)]
#[cfg_attr(test, derive(BorshSerialize))]
pub struct CreateParams {
    pub project: ProjectInfo,
    pub address: Pubkey,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone)]
pub struct ComplimentParams {
    address: Pubkey,
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

                let (mut bank_address_size_bytes, rest) = rest.split_at(4);
                let bank_address_size =
                    <u32 as BorshDeserialize>::deserialize(&mut bank_address_size_bytes)
                        .map_err(|_| InvalidInstruction)? as usize;

                let (mut bank_address_bytes, rest) = rest.split_at(bank_address_size);
                let bank_address =
                    <Pubkey as BorshDeserialize>::deserialize(&mut bank_address_bytes)
                        .map_err(|_| InvalidInstruction)?;

                let (mut owner_address_size_bytes, rest) = rest.split_at(4);
                let owner_address_size =
                    <u32 as BorshDeserialize>::deserialize(&mut owner_address_size_bytes)
                        .map_err(|_| InvalidInstruction)? as usize;

                let (mut owner_address_bytes, rest) = rest.split_at(owner_address_size);
                let owner_address =
                    <Pubkey as BorshDeserialize>::deserialize(&mut owner_address_bytes)
                        .map_err(|_| InvalidInstruction)?;

                let (mut name_length_bytes, rest) = rest.split_at(4);
                let name_length = <u32 as BorshDeserialize>::deserialize(&mut name_length_bytes)
                    .map_err(|_| InvalidInstruction)? as usize;

                let (name_bytes, rest) = rest.split_at(name_length);
                let name =
                    String::from_utf8(name_bytes.to_vec()).map_err(|_| InvalidInstruction)?;

                let (mut project_address_size_bytes, rest) = rest.split_at(4);
                let project_address_size =
                    <u32 as BorshDeserialize>::deserialize(&mut project_address_size_bytes)
                        .map_err(|_| InvalidInstruction)? as usize;

                let (mut project_address, _) = rest.split_at(project_address_size);
                let project_address =
                    <Pubkey as BorshDeserialize>::deserialize(&mut project_address)
                        .map_err(|_| InvalidInstruction)?;

                instruction = Instruction::Create {
                    params: CreateParams {
                        project: ProjectInfo {
                            target,
                            bank: bank_address,
                            owner: owner_address,
                            name,
                        },
                        address: project_address,
                    },
                };
            }

            _ => {
                return Err(InvalidInstruction);
            }
        }

        Ok(instruction)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_instructions() {
        let print = |ins: Instruction| {
            let mut serialized = Vec::<u8>::new();
            <Instruction as BorshSerialize>::serialize(&ins, &mut serialized).unwrap();
            println!("{:#?} is {:?}", ins, serialized);
            println!("=====================");
        };

        let instructions = [Instruction::Create {
            params: CreateParams {
                project: Default::default(),
                address: Pubkey::new_unique(),
            },
        }];

        for ins in instructions {
            print(ins);
        }
    }
}
