use moss_global_env::models::types::VariableInfo;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use ts_rs::TS;

pub type EnvironmentName = String;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct WorkspaceInfo {
    pub path: PathBuf,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct CollectionInfo {
    // pub path: PathBuf,
    pub key: u64,
    pub name: String,
    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfo {
    pub key: u64,
    pub collection_key: Option<u64>,
    pub name: String,
    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfoFull {
    pub key: u64,
    pub name: String,
    pub order: Option<usize>,
    pub variables: HashMap<String, VariableInfo>,
}
