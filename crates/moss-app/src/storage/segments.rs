use moss_storage::primitives::segkey::{SegKey, SegKeyBuf};
use sapic_window::types::primitives::WorkspaceId;

pub static SEGKEY_LAST_ACTIVE_WORKSPACE: SegKey = SegKey::new("lastActiveWorkspace");
pub static SEGKEY_WORKSPACE: SegKey = SegKey::new("workspace");

pub fn segkey_last_opened_at(id: &WorkspaceId) -> SegKeyBuf {
    SEGKEY_WORKSPACE
        .to_segkey_buf()
        .join(id)
        .join("lastOpenedAt")
}

pub fn segkey_workspace(id: &WorkspaceId) -> SegKeyBuf {
    SEGKEY_WORKSPACE.join(id)
}
