pub mod configuration;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;
use validator::Validate;

use crate::models::{
    primitives::{EntryClass, EntryKind, EntryPath, EntryProtocol},
    types::configuration::{CompositeDirConfigurationModel, CompositeItemConfigurationModel},
};

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

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateItemEntryParams {
    pub id: Uuid,
    // TODO: Add validation for path
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub protocol: Option<EntryProtocol>,
    pub order: Option<usize>,
    pub expanded: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateDirEntryParams {
    pub id: Uuid,

    // TODO: Add validation for path
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub order: Option<usize>,
    pub expanded: Option<bool>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AfterUpdateDirEntryDescription {
    pub id: Uuid,
    pub path: EntryPath,
    pub configuration: CompositeDirConfigurationModel,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AfterUpdateItemEntryDescription {
    pub id: Uuid,
    pub path: EntryPath,
    pub configuration: CompositeItemConfigurationModel,
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
