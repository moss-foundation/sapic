use crate::{Error, ErrorMarker, OptionExt, ResultExt};
use anyhow::anyhow;

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new::<()>(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::new::<()>(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Error::new::<()>("mutex poisoned").join::<()>(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::new::<()>(err.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::new::<()>(err.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::new::<()>(err.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::new::<()>(err.to_string())
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::new::<()>(err.to_string())
    }
}

impl From<Error> for anyhow::Error {
    fn from(err: Error) -> Self {
        anyhow!(err.to_string())
    }
}

impl<T> ResultExt<T> for Result<T, Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| e.join::<E>(details.into()))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| e.join::<E>(details()))
    }
}

impl<T> ResultExt<T> for anyhow::Result<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

impl<T> ResultExt<T> for Result<T, serde_json::Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

// FIXME: Not sure if this is the best place to implement this trait
// Maybe it would be better to be feature-gated
impl<T> ResultExt<T> for tokio::io::Result<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }
    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

impl<T> ResultExt<T> for Result<T, git2::Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_join_err<E: ErrorMarker>(self, details: impl Into<String>) -> crate::Result<T> {
        self.ok_or(Error::new::<E>(details.into()))
    }

    fn ok_or_join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> crate::Result<T> {
        self.ok_or_else(|| Error::new::<E>(details()))
    }
}
