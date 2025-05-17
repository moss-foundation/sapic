use moss_common::models::primitives::Identifier;
use moss_environment::models::types::VariableInfo;
use serde::{Deserialize, Serialize};
use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

use ts_rs::TS;
use validator::Validate;

use crate::models::types::{
    CollectionInfo, EditorPartState, EnvironmentInfo, PanelPartState, SidebarPartState,
};

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionInput {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionOutput {
    #[ts(type = "Identifier")]
    pub id: Identifier,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateCollectionEntryInput {
    #[ts(type = "Identifier")]
    pub id: Identifier,

    #[validate(length(min = 1))]
    pub new_name: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateCollectionEntryOutput {
    #[ts(type = "Identifier")]
    pub id: Identifier,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteCollectionInput {
    #[ts(type = "Identifier")]
    pub id: Identifier,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteCollectionOutput {
    #[ts(type = "Identifier")]
    pub id: Identifier,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
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
    #[ts(type = "Identifier")]
    pub id: Identifier,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentOutput {
    #[ts(type = "VariableInfo")]
    pub variables: Vec<VariableInfo>,
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
