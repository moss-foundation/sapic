use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct TauriError(pub String);

impl std::fmt::Display for TauriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<anyhow::Error> for TauriError {
    fn from(e: anyhow::Error) -> Self {
        TauriError(e.to_string())
    }
}

impl From<serde_json::Error> for TauriError {
    fn from(e: serde_json::Error) -> Self {
        TauriError(e.to_string())
    }
}

impl From<moss_common::api::OperationError> for TauriError {
    fn from(e: moss_common::api::OperationError) -> Self {
        TauriError(e.to_string())
    }
}

impl<E: std::fmt::Display> From<moss_app::context::TaskError<E>> for TauriError {
    fn from(err: moss_app::context::TaskError<E>) -> Self {
        match err {
            moss_app::context::TaskError::Err(e) => TauriError(e.to_string()),
            moss_app::context::TaskError::Timeout => TauriError("Task timed out".to_string()),
            moss_app::context::TaskError::Cancelled => TauriError("Task was cancelled".to_string()),
        }
    }
}

pub type TauriResult<T> = Result<T, TauriError>;
