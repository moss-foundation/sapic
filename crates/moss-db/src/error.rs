use joinerror::error::ErrorMarker;
use sapic_core::context::Reason;
use std::io::ErrorKind;
use thiserror::Error;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("entity with key {key} is not found")]
    NotFound { key: String },

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("transaction error: {0}")]
    Transaction(String),

    #[error("canceled: {0}")]
    Canceled(Reason),

    #[error("timeout: {0}")]
    Timeout(String),

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl From<redb::Error> for DatabaseError {
    fn from(error: redb::Error) -> Self {
        DatabaseError::Unknown(anyhow::anyhow!(error))
    }
}

impl From<redb::TableError> for DatabaseError {
    fn from(error: redb::TableError) -> Self {
        DatabaseError::Internal(error.to_string())
    }
}

impl From<redb::StorageError> for DatabaseError {
    fn from(error: redb::StorageError) -> Self {
        DatabaseError::Internal(error.to_string())
    }
}

impl From<redb::TransactionError> for DatabaseError {
    fn from(error: redb::TransactionError) -> Self {
        DatabaseError::Transaction(error.to_string())
    }
}

impl From<redb::CommitError> for DatabaseError {
    fn from(error: redb::CommitError) -> Self {
        DatabaseError::Internal(error.to_string())
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(error: serde_json::Error) -> Self {
        DatabaseError::Serialization(error.to_string())
    }
}
impl From<DatabaseError> for std::io::Error {
    fn from(error: DatabaseError) -> Self {
        std::io::Error::new(
            ErrorKind::Other,
            format!("Database operation failed: {}", error.to_string()),
        )
    }
}

impl From<DatabaseError> for joinerror::Error {
    fn from(error: DatabaseError) -> Self {
        joinerror::Error::new::<()>(error.to_string())
    }
}

pub trait DbResultExt<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T>;

    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T>;
}

impl<T> DbResultExt<T> for DatabaseResult<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()).join_with::<E>(details))
    }
}
