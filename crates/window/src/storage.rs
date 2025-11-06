// Segments are joined using .
use moss_logging::models::primitives::LogEntryId;

use crate::models::primitives::WorkspaceId;

pub static KEY_LAST_ACTIVE_WORKSPACE: &'static str = "lastActiveWorkspace";

pub static KEY_WORKSPACE_PREFIX: &'static str = "workspace";
pub static KEY_LOG_PREFIX: &'static str = "log";

pub fn key_workspace_last_opened_at(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}.lastOpenedAt", id.to_string())
}

pub fn key_workspace(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}", id.to_string())
}

pub fn key_log(id: &LogEntryId) -> String {
    format!("{KEY_LOG_PREFIX}.{}", id.to_string())
}
