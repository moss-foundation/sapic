use moss_common::leased_slotmap::ResourceKey;
use moss_environment::models::types::VariableInfo;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

use super::types::{CollectionInfo, EnvironmentInfo, WorkspaceInfo};

// ------------------------------------------------------------------
// Workspace Manager operations
// ------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
#[serde(rename_all = "camelCase")]
pub struct OpenWorkspaceInput {
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

// ------------------------------------------------------------------
// Workspace operations
// ------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListCollectionsOutput(pub Vec<CollectionInfo>);

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
#[serde(rename_all = "camelCase")]
pub struct ListCollectionRequestsInput {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionInput {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionOutput {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RenameCollectionInput {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
    #[validate(length(min = 1))]
    pub new_name: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RenameCollectionOutput {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteCollectionInput {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeWorkspaceOutput {
    pub collections: Vec<CollectionInfo>,
    pub environments: Vec<EnvironmentInfo>,
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentInput {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentOutput {
    #[ts(type = "VariableInfo")]
    pub variables: Vec<VariableInfo>,
}
