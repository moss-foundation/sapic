use redb::{ReadTransaction as InnerReadTransaction, WriteTransaction as InnerWriteTransaction};
use std::io::{Error, ErrorKind};
use thiserror::Error;

pub type AnyEntity = Vec<u8>;

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

pub enum Transaction {
    Read(InnerReadTransaction),
    Write(InnerWriteTransaction),
}

impl Transaction {
    pub fn commit(self) -> anyhow::Result<(), DatabaseError> {
        match self {
            Transaction::Read(_) => Ok(()),
            Transaction::Write(txn) => Ok(txn.commit()?),
        }
    }
}
