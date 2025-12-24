// Segments are joined using .

use sapic_base::workspace::types::primitives::WorkspaceId;

pub static KEY_WORKSPACE_PREFIX: &'static str = "workspace";

pub fn key_workspace_last_opened_at(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}.lastOpenedAt", id.to_string())
}

pub fn key_workspace(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}", id.to_string())
}
