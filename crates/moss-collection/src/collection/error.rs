use moss_db::common::DatabaseError;
use std::path::PathBuf;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("{name} not found at {path}")]
    NotFound { name: String, path: PathBuf },

    #[error("{name} already exists at {path}")]
    AlreadyExists { name: String, path: PathBuf },

    #[error("internal error: {0}")]
    Internal(String),

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
    // FIXME: Should we have an error for incorrect entity type?
}

impl From<DatabaseError> for OperationError {
    fn from(error: DatabaseError) -> Self {
        OperationError::Internal(error.to_string())
    }
}
