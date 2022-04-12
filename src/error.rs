use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrowdError {
    #[error("Unexpected instruction")]
    InvalidInstruction = 0,

    #[error("No such project exists")]
    InvalidProject = 2,
}

impl From<CrowdError> for ProgramError {
    fn from(crowd_error: CrowdError) -> Self {
        ProgramError::Custom(crowd_error as u32)
    }
}

impl TryFrom<u32> for CrowdError {
    type Error = ();

    fn try_from(error_code: u32) -> Result<CrowdError, Self::Error> {
        use CrowdError::*;

        let error = match error_code {
            1 => InvalidInstruction,
            2 => InvalidProject,
            _ => return Err(()),
        };

        Ok(error)
    }
}
