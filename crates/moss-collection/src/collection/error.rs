use std::path::PathBuf;
use moss_db::common::DatabaseError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("{name} not found at {path}")]
    NotFound { name: String, path: PathBuf },

    #[error("{name} already exists at {path}")]
    AlreadyExists { name: String, path: PathBuf },

    #[error("internal error: {0}")]
    Internal(String),

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl From<DatabaseError> for OperationError {
    fn from(error: DatabaseError) -> Self {
        OperationError::Internal(error.to_string())
    }
}
