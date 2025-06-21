use moss_environment::models::types::VariableInfo;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use url::Url;
use uuid::Uuid;
use validator::Validate;

use crate::models::types::{CollectionInfo, EditorPartStateInfo, EnvironmentInfo};

use super::types::{ActivitybarPartStateInfo, PanelPartStateInfo, SidebarPartStateInfo};

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionInput {
    #[validate(length(min = 1))]
    pub name: String,

    #[ts(optional)]
    pub order: Option<usize>,

    #[ts(optional)]
    pub external_path: Option<PathBuf>,

    #[ts(optional)]
    pub repo: Option<Url>,

    #[ts(optional)]
    pub icon_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionOutput {
    pub id: Uuid,
    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum ChangeInput<T> {
    Update(T),
    Remove,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateCollectionInput {
    pub id: Uuid,

    #[validate(length(min = 1))]
    #[ts(optional)]
    pub new_name: Option<String>,

    #[ts(optional)]
    pub new_repo: Option<ChangeInput<Url>>,

    #[ts(optional)]
    pub new_icon: Option<ChangeInput<PathBuf>>,

    #[ts(optional)]
    pub order: Option<usize>,

    #[ts(optional)]
    pub pinned: Option<bool>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateCollectionOutput {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteCollectionInput {
    pub id: Uuid,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteCollectionOutput {
    pub id: Uuid,

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
    pub id: Uuid,
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
    pub editor: Option<EditorPartStateInfo>,
    #[ts(optional)]
    pub sidebar: Option<SidebarPartStateInfo>,
    #[ts(optional)]
    pub panel: Option<PanelPartStateInfo>,
    #[ts(optional)]
    pub activitybar: Option<ActivitybarPartStateInfo>,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateStateInput {
    UpdateEditorPartState(EditorPartStateInfo),
    UpdateSidebarPartState(SidebarPartStateInfo),
    UpdatePanelPartState(PanelPartStateInfo),
    UpdateActivitybarPartState(ActivitybarPartStateInfo),
}
