use moss_storage::primitives::segkey::{SegKey, SegKeyBuf};

pub fn segkey_entry_order(id: &str) -> SegKeyBuf {
    SEGKEY_RESOURCE_ENTRY.join(id).join("order")
}

pub static SEGKEY_RESOURCE_ENTRY: SegKey = SegKey::new("entry");
pub static SEGKEY_RESOURCE_ENVIRONMENT: SegKey = SegKey::new("env");
