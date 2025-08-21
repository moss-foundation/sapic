use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{
    primitives::{EntryClass, EntryId, EntryKind, EntryProtocol, FrontendEntryPath},
    types::{AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription},
};

/// @category Event
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEntriesEvent {
    /// Unique identifier for this entry
    pub id: EntryId,

    /// Display name of the entry
    pub name: String,

    /// Path relative to the collection root.
    /// Includes both the original path string and its segments.
    pub path: FrontendEntryPath,

    /// Classification of the entry (Request, Endpoint, Component, or Schema)
    pub class: EntryClass,

    /// Type of entry indicating its structure (Dir for directories, Item for files, Case of item cases)
    pub kind: EntryKind,

    /// HTTP protocol/method used by this entry, if applicable (GET, POST, PUT, DELETE, WebSocket, GraphQL, gRPC)
    pub protocol: Option<EntryProtocol>,

    /// Determines the display position of this entry among others in the same group.
    /// Entries are sorted in ascending order; lower values appear before higher ones.
    /// Negative values are allowed and will be placed before positive values.
    /// If multiple entries have the same order, they are sorted alphabetically.
    /// If not specified, the entry appears last and is sorted alphabetically
    /// among unspecified items.
    pub order: Option<isize>,

    /// Whether this entry is expanded in the tree view (applies to directories)
    pub expanded: bool,
}

/// @category Event
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "events.ts")]
pub enum BatchUpdateEntryEvent {
    Item(AfterUpdateItemEntryDescription),
    Dir(AfterUpdateDirEntryDescription),
}
