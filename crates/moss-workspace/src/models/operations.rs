use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::types::WorkspaceInfo;

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
#[serde(rename_all = "camelCase")]
pub struct SetWorkspaceInput {
    pub path: PathBuf,
}
