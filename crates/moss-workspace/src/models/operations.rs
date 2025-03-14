use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

use super::types::{CollectionInfo, WorkspaceInfo};

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
#[serde(rename_all = "camelCase")]
pub struct SetWorkspaceInput {
    pub path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListCollectionsOutput(pub Vec<CollectionInfo>);

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeCollectionOutput {}
