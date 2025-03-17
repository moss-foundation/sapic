use moss_collection::models::collection::RequestType;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, path::PathBuf};
use ts_rs::TS;

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
pub struct RequestInfo {
    pub name: String,
    #[ts(optional)]
    pub order: Option<usize>,
    pub typ: RequestType,
}
