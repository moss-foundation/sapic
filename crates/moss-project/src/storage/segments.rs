use moss_storage::primitives::segkey::{SegKey, SegKeyBuf};

use crate::models::primitives::{EntryId, HeaderId, PathParamId, QueryParamId};

pub static SEGKEY_RESOURCE_ENTRY: SegKey = SegKey::new("entry");
pub static SEGKEY_EXPANDED_ENTRIES: SegKey = SegKey::new("expandedEntries");

pub fn segkey_entry_order(id: &EntryId) -> SegKeyBuf {
    SEGKEY_RESOURCE_ENTRY.join(id).join("order")
}

pub fn segkey_entry_header_order(entry_id: &EntryId, header_id: &HeaderId) -> SegKeyBuf {
    SEGKEY_RESOURCE_ENTRY
        .join(entry_id)
        .join("header")
        .join(header_id)
        .join("order")
}

pub fn segkey_entry_path_param_order(entry_id: &EntryId, path_param_id: &PathParamId) -> SegKeyBuf {
    SEGKEY_RESOURCE_ENTRY
        .join(entry_id)
        .join("path_param")
        .join(path_param_id)
        .join("order")
}

pub fn segkey_entry_query_param_order(
    entry_id: &EntryId,
    query_param_id: &QueryParamId,
) -> SegKeyBuf {
    SEGKEY_RESOURCE_ENTRY
        .join(entry_id)
        .join("query_param")
        .join(query_param_id)
        .join("order")
}
