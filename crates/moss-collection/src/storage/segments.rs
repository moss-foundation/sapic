use moss_common::NanoId;
use moss_storage::primitives::segkey::{SegKey, SegKeyBuf};

pub static SEGKEY_RESOURCE_ENTRY: SegKey = SegKey::new("entry");
pub static SEGKEY_EXPANDED_ENTRIES: SegKey = SegKey::new("expandedEntries");
pub static SEGKEY_RESOURCE_ENVIRONMENT: SegKey = SegKey::new("env");

pub fn segkey_entry_order(id: &NanoId) -> SegKeyBuf {
    SEGKEY_RESOURCE_ENTRY.join(id).join("order")
}
