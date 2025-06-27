pub mod configuration;

use serde::Serialize;
use std::path::PathBuf;
use ts_rs::TS;
use uuid::Uuid;

use crate::models::primitives::{EntryClass, EntryKind, EntryProtocol};

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfo {
    pub id: Uuid,
    pub name: String,
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EntryPath {
    pub raw: PathBuf,
    pub segments: Vec<String>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EntryInfo {
    pub id: Uuid,
    pub name: String,
    pub path: EntryPath,
    pub class: EntryClass,
    pub kind: EntryKind,
    pub protocol: Option<EntryProtocol>,
    pub order: Option<usize>,
    pub expanded: bool,
}
