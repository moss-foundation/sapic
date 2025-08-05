use joinerror::error::ErrorMarker;
use std::{io, io::ErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Permission Denied: {0}")]
    PermissionDenied(String),
    #[error("Already Exists: {0}")]
    AlreadyExists(String),
    #[error("Other: {0}")]
    Other(String),
}
impl From<io::Error> for FsError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound => Self::NotFound(error.to_string()),
            ErrorKind::PermissionDenied => Self::PermissionDenied(error.to_string()),
            ErrorKind::AlreadyExists => Self::AlreadyExists(error.to_string()),
            _ => Self::Other(error.to_string()),
        }
    }
}

impl From<notify::Error> for FsError {
    fn from(error: notify::Error) -> Self {
        // FIXME: how to best handle watcher error?
        FsError::Other(error.to_string())
    }
}

impl From<anyhow::Error> for FsError {
    fn from(error: anyhow::Error) -> Self {
        FsError::Other(error.to_string())
    }
}

impl From<FsError> for joinerror::Error {
    fn from(error: FsError) -> Self {
        joinerror::Error::new::<()>(error.to_string())
    }
}

pub type FsResult<T> = Result<T, FsError>;

pub trait FsResultExt<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T>;

    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T>;
}

impl<T> FsResultExt<T> for FsResult<T> {
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
