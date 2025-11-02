pub mod models;

pub mod constants {
    pub const APP_SCOPE: &'static str = "app";
    pub const SESSION_SCOPE: &'static str = "session";
}

pub struct LogEvent {
    // FIXME: We might want to support a set of identifiers
    // Including CollectionID, EntryID and EnvironmentID
    pub resource: Option<String>,
    pub message: String,
}

pub enum LogScope {
    App,
    Session,
}

/// Extension trait for logging errors in Result types.
pub trait ResultSessionLogExt {
    /// Log error if Result is Err, using session scope at error level.
    #[track_caller]
    fn log_err(self, context: &str) -> Self;

    /// Log error as warning in session scope
    #[track_caller]
    fn log_warn(self, context: &str) -> Self;

    /// Log error as info in session scope
    #[track_caller]
    fn log_info(self, context: &str) -> Self;

    /// Log error as debug in session scope
    #[track_caller]
    fn log_debug(self, context: &str) -> Self;
}

impl<T> ResultSessionLogExt for joinerror::Result<T> {
    #[track_caller]
    fn log_err(self, context: &str) -> Self {
        if let Err(ref e) = self {
            let location = std::panic::Location::caller();
            let location_str = format!("{}:{}", location.file(), location.line());
            session::error!(format!(
                "Operation failed at {}: {} - {}",
                location_str, context, e
            ));
        }
        self
    }

    #[track_caller]
    fn log_warn(self, context: &str) -> Self {
        if let Err(ref e) = self {
            let location = std::panic::Location::caller();
            session::warn!(format!(
                "{} at {}:{} - {}",
                context,
                location.file(),
                location.line(),
                e
            ));
        }
        self
    }

    #[track_caller]
    fn log_info(self, context: &str) -> Self {
        if let Err(ref e) = self {
            let location = std::panic::Location::caller();
            session::info!(format!(
                "{} at {}:{} - {}",
                context,
                location.file(),
                location.line(),
                e
            ));
        }
        self
    }

    #[track_caller]
    fn log_debug(self, context: &str) -> Self {
        if let Err(ref e) = self {
            let location = std::panic::Location::caller();
            session::debug!(format!(
                "{} at {}:{} - {}",
                context,
                location.file(),
                location.line(),
                e
            ));
        }
        self
    }
}

pub mod app {
    #[macro_export]
    macro_rules! trace_app {
        // Rule for `app::trace!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::trace!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `app::trace!("message")`
        ($message:expr) => {
            tracing::trace!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    #[macro_export]
    macro_rules! debug_app {
        // Rule for `app::debug!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::debug!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `app::debug!("message")`
        ($message:expr) => {
            tracing::debug!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    #[macro_export]
    macro_rules! info_app {
        // Rule for `app::info!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::info!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `app::info!("message")`
        ($message:expr) => {
            tracing::info!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    #[macro_export]
    macro_rules! warn_app {
        // Rule for `app::warn!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::warn!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `app::warn!("message")`
        ($message:expr) => {
            tracing::warn!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    #[macro_export]
    macro_rules! error_app {
        // Rule for `app::error!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::error!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `app::error!("message")`
        ($message:expr) => {
            tracing::error!(
                target: $crate::constants::APP_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    pub use debug_app as debug;
    pub use error_app as error;
    pub use info_app as info;
    pub use trace_app as trace;
    pub use warn_app as warn;
}

pub mod session {
    use super::*;
    #[macro_export]
    macro_rules! trace_session {
        // Rule for `session::trace!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::trace!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `session::trace!("message")`
        ($message:expr) => {
            tracing::trace!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    #[macro_export]
    macro_rules! debug_session {
        // Rule for `session::debug!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::debug!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `session::debug!("message")`
        ($message:expr) => {
            tracing::debug!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    #[macro_export]
    macro_rules! info_session {
        // Rule for `session::info!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::info!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `session::info!("message")`
        ($message:expr) => {
            tracing::info!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    #[macro_export]
    macro_rules! warn_session {
        // Rule for `session::warn!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::warn!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `session::warn!("message")`
        ($message:expr) => {
            tracing::warn!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    #[macro_export]
    macro_rules! error_session {
        // Rule for `session::error!(resource, "message")`
        ($resource:expr, $message:expr) => {
            tracing::error!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                resource = Some($resource),
                message = $message
            )
        };
        // Rule for `session::error!("message")`
        ($message:expr) => {
            tracing::error!(
                target: $crate::constants::SESSION_SCOPE,
                id = $crate::models::primitives::LogEntryId::new().to_string(),
                message = $message
            )
        };
    }

    pub use debug_session as debug;
    pub use error_session as error;
    pub use info_session as info;
    pub use trace_session as trace;
    pub use warn_session as warn;
}
