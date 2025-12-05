use serde::Serialize;
use std::path::PathBuf;
use ts_rs::TS;

// The response from list all extensions and get a particular extension is slightly different
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "extension/types.ts")]
pub struct ExtensionVersionInfo {
    pub id: String,
    pub external_id: String,
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub repository: String,
    pub downloads: u64,
    pub created_at: String,
    pub updated_at: String,
    pub version: String,
}

pub struct LoadedExtensionInfo {
    pub source: PathBuf,
}
