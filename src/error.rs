use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrowdError {
    #[error("Unexpected instruction")]
    InvalidInstruction = 0,

    #[error("No such project exists")]
    InvalidProject,

    #[error("Creator didn't signed this instruction")]
    IllegalCreator,

    #[error("Donator didn't signed this message")]
    IllegalDonator,

    #[error("Bank address was not as expected")]
    UnexpectedBankAddress,

    #[error("This is not the expected project address")]
    UnexpectedProjectAddress,

    #[error("Passed bump value was not as calculated")]
    UnexpectedBump,

    #[error("This project address is already in use")]
    ProjectAddressCollision,

    #[error("This bank address is already in use")]
    BankAddresCollision,

    #[error("This project info is too large to store.")]
    LargeProjectInfo,

    #[error("This project address have malformed data")]
    CorruptedProjectData,
}

impl From<CrowdError> for ProgramError {
    fn from(crowd_error: CrowdError) -> Self {
        ProgramError::Custom(crowd_error as u32)
    }
}
