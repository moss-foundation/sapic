use moss_common::leased_slotmap::ResourceKey;
use moss_environment::models::types::VariableInfo;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use ts_rs::TS;
use validator::Validate;

use crate::models::types::{
    CollectionInfo, EditorPartState, EnvironmentInfo, PanelPartState, SidebarPartState,
};

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

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct OpenCollectionInput {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct OpenCollectionOutput {
    #[ts(type = "ResourceKey")]
    pub key: ResourceKey,
    pub path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeStateOutput {
    #[ts(optional)]
    pub editor: Option<EditorPartState>,
    #[ts(optional)]
    pub sidebar: Option<SidebarPartState>,
    #[ts(optional)]
    pub panel: Option<PanelPartState>,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateStateInput {
    UpdateEditorPartState(EditorPartState),
    UpdateSidebarPartState(SidebarPartState),
    UpdatePanelPartState(PanelPartState),
}
