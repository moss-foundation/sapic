use serde::Serialize;
use std::path::PathBuf;
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "extension/types.ts")]
pub struct ExtensionInfo {
    pub id: String,
    pub external_id: String,
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub repository: String,
    pub downloads: u64,
    pub created_at: String,
    pub updated_at: String,
    pub latest_version: String,
}

pub struct LoadedExtensionInfo {
    pub source: PathBuf,
}
