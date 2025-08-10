use crate::{
    constants::{APP_SCOPE, SESSION_SCOPE},
    models::primitives::LogEntryId,
};
use tracing::{debug, error, info, trace, warn};

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

// Tracing disallows non-constant value for `target`
// So we have to manually match it
pub fn trace(scope: LogScope, payload: LogEvent) {
    let id = LogEntryId::new().to_string();
    match scope {
        LogScope::App => {
            trace!(
                target: APP_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
        LogScope::Session => {
            trace!(
                target: SESSION_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
    }
}

pub fn debug(scope: LogScope, payload: LogEvent) {
    let id = LogEntryId::new().to_string();
    match scope {
        LogScope::App => {
            debug!(
                target: APP_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
        LogScope::Session => {
            debug!(
                target: SESSION_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
    }
}

pub fn info(scope: LogScope, payload: LogEvent) {
    let id = LogEntryId::new().to_string();
    match scope {
        LogScope::App => {
            info!(
                target: APP_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
        LogScope::Session => {
            info!(
                target: SESSION_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
    }
}

pub fn warn(scope: LogScope, payload: LogEvent) {
    let id = LogEntryId::new().to_string();
    match scope {
        LogScope::App => {
            warn!(
                target: APP_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
        LogScope::Session => {
            warn!(
                target: SESSION_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
    }
}

pub fn error(scope: LogScope, payload: LogEvent) {
    let id = LogEntryId::new().to_string();
    match scope {
        LogScope::App => {
            error!(
                target: APP_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
        LogScope::Session => {
            error!(
                target: SESSION_SCOPE,
                id = id,
                resource = payload.resource,
                message = payload.message
            )
        }
    }
}
