pub mod models;
mod utils;

pub use utils::*;

use serde::Serialize;
use thiserror::Error;

pub mod constants {
    use std::time::Duration;

    pub const DEFAULT_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);
}

#[derive(Debug, Error)]
pub enum TauriError {
    #[error(transparent)]
    OperationError(#[from] moss_common::api::OperationError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("Operation timed out")]
    Timeout,
}

impl From<joinerror::Error> for TauriError {
    fn from(error: joinerror::Error) -> Self {
        TauriError::Other(error.into())
    }
}

impl Serialize for TauriError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

pub type TauriResult<T> = Result<T, TauriError>;
