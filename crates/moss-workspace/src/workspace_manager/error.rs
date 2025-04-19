use std::path::PathBuf;

use moss_db::common::DatabaseError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("workspace {name} not found at {path}")]
    NotFound { name: String, path: PathBuf },

    #[error("workspace {name} already exists at {path}")]
    AlreadyExists { name: String, path: PathBuf },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl From<DatabaseError> for OperationError {
    fn from(error: DatabaseError) -> Self {
        OperationError::Unknown(anyhow::anyhow!(error))
    }
}
