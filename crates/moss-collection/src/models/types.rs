pub mod configuration;

use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::models::primitives::{EntryClass, EntryKind, EntryPath, EntryProtocol};

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfo {
    pub id: Uuid,
    pub name: String,

    /// Determines the display position of this entry among others in the same group.
    /// Entries are sorted in ascending order; lower values appear before higher ones.
    /// Negative values are allowed and will be placed before positive values.
    /// If multiple entries have the same order, they are sorted alphabetically.
    /// If not specified, the entry appears last and is sorted alphabetically
    /// among unspecified items.
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EntryInfo {
    /// Unique identifier for this entry
    pub id: Uuid,

    /// Display name of the entry
    pub name: String,

    /// Path relative to the collection root.
    /// Includes both the original path string and its segments.
    pub path: EntryPath,

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
    pub order: Option<usize>,

    /// Whether this entry is expanded in the tree view (applies to directories)
    pub expanded: bool,
}
