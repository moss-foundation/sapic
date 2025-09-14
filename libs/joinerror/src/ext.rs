use crate::{Error, ErrorMarker, OptionExt, ResultExt};

// FIXME: Remove conversion traits for git errors and implement crate specific traits

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
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

impl From<std::path::StripPrefixError> for Error {
    fn from(err: std::path::StripPrefixError) -> Self {
        Error::new::<()>(err.to_string())
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new::<()>(err.to_string())
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::new::<()>(err.to_string())
    }
}

#[cfg(feature = "url")]
impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::new::<()>(err.to_string())
    }
}

#[cfg(feature = "anyhow")]
impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::new::<()>(err.to_string())
    }
}

#[cfg(feature = "tokio")]
impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::new::<()>(err.to_string())
    }
}

#[cfg(feature = "git2")]
impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::new::<()>(err.to_string())
    }
}

impl<T> ResultExt<T> for Result<T, std::path::StripPrefixError> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

impl<T> ResultExt<T> for Result<T, std::string::FromUtf8Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
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

#[cfg(feature = "anyhow")]
impl<T> ResultExt<T> for anyhow::Result<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

#[cfg(feature = "serde_json")]
impl<T> ResultExt<T> for Result<T, serde_json::Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

#[cfg(feature = "tokio")]
impl<T> ResultExt<T> for tokio::io::Result<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }
    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

#[cfg(feature = "git2")]
impl<T> ResultExt<T> for Result<T, git2::Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(details()))
    }
}

#[cfg(feature = "reqwest")]
impl<T> ResultExt<T> for reqwest::Result<T> {
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
