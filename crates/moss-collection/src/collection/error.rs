use std::path::PathBuf;

use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("request {name} not found at {path}")]
    RequestNotFound { name: String, path: PathBuf },

    #[error("request group {path} not found")]
    RequestGroupNotFound { path: PathBuf },

    #[error("request {name} already exists at {path}")]
    RequestAlreadyExists { name: String, path: PathBuf },

    #[error("request group {path} already exists")]
    RequestGroupAlreadyExists { path: PathBuf },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}
