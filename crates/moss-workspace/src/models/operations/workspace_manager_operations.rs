use std::path::PathBuf;

use moss_common::leased_slotmap::ResourceKey;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::models::types::WorkspaceInfo;

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
#[serde(rename_all = "camelCase")]
pub struct OpenWorkspaceInput {
    /// We use the workspace name instead of its path because
    /// all workspaces can only be stored within a single directory.
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
#[serde(rename_all = "camelCase")]
pub struct OpenWorkspaceOutput {
    pub path: PathBuf,
}

#[derive(Debug, Validate, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspaceInput {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Debug, Validate, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspaceOutput {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
    pub path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteWorkspaceInput {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
}

#[derive(Debug, Validate, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
#[serde(rename_all = "camelCase")]
pub struct RenameWorkspaceInput {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
    #[validate(length(min = 1))]
    pub new_name: String,
}
