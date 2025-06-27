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

use crate::models::types::{EditorPartStateInfo, EnvironmentInfo};

use super::types::{ActivitybarPartStateInfo, PanelPartStateInfo, SidebarPartStateInfo};

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionInput {
    #[validate(length(min = 1))]
    pub name: String,

    pub order: Option<usize>,
    pub external_path: Option<PathBuf>,
    pub repo: Option<Url>,
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
pub enum ChangeRepository {
    Update(Url),
    Remove,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum ChangeIcon {
    Update(PathBuf),
    Remove,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateCollectionInput {
    pub id: Uuid,

    #[validate(length(min = 1))]
    pub new_name: Option<String>,

    pub new_repo: Option<ChangeRepository>,
    pub new_icon: Option<ChangeIcon>,
    pub order: Option<usize>,
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
