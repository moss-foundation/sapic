use moss_storage::primitives::segkey::{SegKey, SegKeyBuf};

use crate::models::primitives::EntryId;

pub static SEGKEY_RESOURCE_ENTRY: SegKey = SegKey::new("entry");
pub static SEGKEY_EXPANDED_ENTRIES: SegKey = SegKey::new("expandedEntries");

pub fn segkey_entry_order(id: &EntryId) -> SegKeyBuf {
    SEGKEY_RESOURCE_ENTRY.join(id).join("order")
}
