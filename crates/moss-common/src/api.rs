use std::path::PathBuf;
use thiserror::Error;

// TODO: add PreconditionFailed
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

impl From<serde_json::Error> for OperationError {
    fn from(error: serde_json::Error) -> Self {
        OperationError::Internal(error.to_string())
    }
}

pub type OperationResult<T> = Result<T, OperationError>;

pub trait OperationResultExt<T> {
    fn map_err_as_internal(self) -> OperationResult<T>;
    fn map_err_as_not_found(self) -> OperationResult<T>;
    fn map_err_as_validation(self) -> OperationResult<T>;
}

impl<T> OperationResultExt<T> for Result<T, anyhow::Error> {
    fn map_err_as_internal(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::Internal(e.to_string()))
    }

    fn map_err_as_not_found(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::NotFound {
            name: e.to_string(),
            path: PathBuf::new(),
        })
    }

    fn map_err_as_validation(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::Validation(e.to_string()))
    }
}

impl<T> OperationResultExt<T> for Result<T, String> {
    fn map_err_as_internal(self) -> OperationResult<T> {
        self.map_err(OperationError::Internal)
    }

    fn map_err_as_not_found(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::NotFound {
            name: e,
            path: PathBuf::new(),
        })
    }

    fn map_err_as_validation(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::Validation(e))
    }
}

impl<T> OperationResultExt<T> for Result<T, &str> {
    fn map_err_as_internal(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::Internal(e.to_string()))
    }

    fn map_err_as_not_found(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::NotFound {
            name: e.to_string(),
            path: PathBuf::new(),
        })
    }

    fn map_err_as_validation(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::Validation(e.to_string()))
    }
}
