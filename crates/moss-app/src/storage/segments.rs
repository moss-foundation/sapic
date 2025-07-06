use moss_storage::primitives::segkey::{SegKey, SegKeyBuf};

pub static SEGKEY_LAST_ACTIVE_WORKSPACE: SegKey = SegKey::new("lastActiveWorkspace");
pub static SEGKEY_WORKSPACE: SegKey = SegKey::new("workspace");

pub fn segkey_last_opened_at(id: &str) -> SegKeyBuf {
    SEGKEY_WORKSPACE
        .to_segkey_buf()
        .join(id)
        .join("lastOpenedAt")
}

pub fn segkey_workspace(id: &str) -> SegKeyBuf {
    SEGKEY_WORKSPACE.to_segkey_buf().join(id)
}
