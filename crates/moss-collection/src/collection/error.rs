use std::path::PathBuf;

use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("request {name} not found at {path}")]
    NotFound { name: String, path: PathBuf },

    #[error("request {name} already exists at {path}")]
    AlreadyExists { name: String, path: PathBuf },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}
