mod editor;

pub use editor::*;

use moss_common::leased_slotmap::ResourceKey;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

pub type EnvironmentName = String;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct WorkspaceInfo {
    pub path: PathBuf,
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct CollectionInfo {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
    pub name: String,
    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfo {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
    #[ts(optional)]
    pub collection_key: Option<ResourceKey>,
    pub name: String,
    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct SidebarPartState {
    pub preferred_size: usize,
    pub is_visible: bool,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PanelPartState {
    pub preferred_size: usize,
    pub is_visible: bool,
}
