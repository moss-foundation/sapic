use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("validation error: {0}")]
    Validation(String), // TODO: rename InvalidInput

    #[error("{name} not found at {path}")]
    NotFound { name: String, path: PathBuf }, // TODO: should be just a string

    #[error("{name} already exists at {path}")]
    AlreadyExists { name: String, path: PathBuf },

    #[error("internal error: {0}")]
    Internal(String),

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl From<moss_db::common::DatabaseError> for OperationError {
    fn from(error: moss_db::common::DatabaseError) -> Self {
        OperationError::Internal(error.to_string())
    }
}

impl From<validator::ValidationErrors> for OperationError {
    fn from(error: validator::ValidationErrors) -> Self {
        OperationError::Validation(error.to_string())
    }
}

impl From<validator::ValidationError> for OperationError {
    fn from(error: validator::ValidationError) -> Self {
        OperationError::Validation(error.to_string())
    }
}

impl From<tauri::Error> for OperationError {
    fn from(error: tauri::Error) -> Self {
        OperationError::Internal(error.to_string())
    }
}

pub type OperationResult<T> = Result<T, OperationError>;
