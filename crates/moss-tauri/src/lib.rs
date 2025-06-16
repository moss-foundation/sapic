use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TauriError {
    #[error(transparent)]
    OperationError(#[from] moss_common::api::OperationError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("Operation timed out")]
    Timeout,
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
