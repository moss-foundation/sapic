pub mod models;
mod utils;

use axum::{http::StatusCode, response::IntoResponse};
pub use utils::*;

use serde::Serialize;
use thiserror::Error;

pub mod constants {
    use std::time::Duration;

    pub const DEFAULT_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);
}

#[derive(Debug, Error)]
pub enum TauriError {
    #[error(transparent)]
    OperationError(#[from] moss_common::api::OperationError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("Operation timed out")]
    Timeout,
}

impl Serialize for TauriError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl IntoResponse for TauriError {
    fn into_response(self) -> axum::response::Response {
        // TODO: More sophisticated error status code
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

pub type TauriResult<T> = Result<T, TauriError>;
