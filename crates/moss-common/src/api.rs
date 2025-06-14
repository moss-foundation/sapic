use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    /// The operation was rejected because the system is not in a state required
    /// for the operation's execution. For example, the workspace to be described or
    /// deleted does not opened yet, etc.
    #[error("FAILED_PRECONDITION: {0}")]
    FailedPrecondition(String),

    /// The operation was rejected because the input was invalid.
    #[error("INVALID_INPUT: {0}")]
    InvalidInput(String),

    /// The entity that a client attempted to access (e.g., file or directory) does not exist.
    #[error("NOT_FOUND: {0}")]
    NotFound(String),

    /// The entity that a client attempted to create (e.g., file or directory) already exists.
    #[error("ALREADY_EXISTS: {0}")]
    AlreadyExists(String),

    /// This means that some invariants expected by the underlying system have been broken.
    /// This error code is reserved for serious errors.
    #[error("INTERNAL: {0}")]
    Internal(String),

    /// This error code is reserved for errors that are not covered by the other error codes.
    #[error("UNKNOWN: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl From<moss_db::common::DatabaseError> for OperationError {
    fn from(error: moss_db::common::DatabaseError) -> Self {
        OperationError::Internal(error.to_string())
    }
}

impl From<validator::ValidationErrors> for OperationError {
    fn from(error: validator::ValidationErrors) -> Self {
        OperationError::InvalidInput(error.to_string())
    }
}

impl From<validator::ValidationError> for OperationError {
    fn from(error: validator::ValidationError) -> Self {
        OperationError::InvalidInput(error.to_string())
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

impl From<toml::ser::Error> for OperationError {
    fn from(error: toml::ser::Error) -> Self {
        OperationError::Internal(error.to_string())
    }
}

impl From<toml::de::Error> for OperationError {
    fn from(error: toml::de::Error) -> Self {
        OperationError::Internal(error.to_string())
    }
}

pub type OperationResult<T> = Result<T, OperationError>;

pub trait OperationResultExt<T> {
    fn map_err_as_internal(self) -> OperationResult<T>;
    fn map_err_as_not_found(self) -> OperationResult<T>;
    fn map_err_as_validation(self) -> OperationResult<T>;
    fn map_err_as_failed_precondition(self) -> OperationResult<T>;
}

impl<T> OperationResultExt<T> for Result<T, anyhow::Error> {
    fn map_err_as_internal(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::Internal(e.to_string()))
    }

    fn map_err_as_not_found(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::NotFound(e.to_string()))
    }

    fn map_err_as_validation(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::InvalidInput(e.to_string()))
    }

    fn map_err_as_failed_precondition(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::FailedPrecondition(e.to_string()))
    }
}

impl<T> OperationResultExt<T> for Result<T, String> {
    fn map_err_as_internal(self) -> OperationResult<T> {
        self.map_err(OperationError::Internal)
    }

    fn map_err_as_not_found(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::NotFound(e))
    }

    fn map_err_as_validation(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::InvalidInput(e))
    }

    fn map_err_as_failed_precondition(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::FailedPrecondition(e))
    }
}

impl<T> OperationResultExt<T> for Result<T, &str> {
    fn map_err_as_internal(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::Internal(e.to_string()))
    }

    fn map_err_as_not_found(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::NotFound(e.to_string()))
    }

    fn map_err_as_validation(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::InvalidInput(e.to_string()))
    }

    fn map_err_as_failed_precondition(self) -> OperationResult<T> {
        self.map_err(|e| OperationError::FailedPrecondition(e.to_string()))
    }
}
