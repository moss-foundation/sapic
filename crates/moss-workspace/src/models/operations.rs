use moss_environment::models::types::VariableInfo;
use moss_git::url::GIT_URL_REGEX;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::models::types::EditorPartStateInfo;

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
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repo: Option<String>,
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
    Update(String),
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

    #[validate(custom(function = "validate_change_repository"))]
    pub new_repo: Option<ChangeRepository>,
    pub new_icon: Option<ChangeIcon>,
    pub order: Option<usize>,
    pub pinned: Option<bool>,
}

fn validate_change_repository(repo: &ChangeRepository) -> Result<(), ValidationError> {
    match repo {
        ChangeRepository::Update(repo) => GIT_URL_REGEX
            .is_match(repo)
            .then_some(())
            .ok_or(ValidationError::new("Invalid Git URL format")),
        ChangeRepository::Remove => Ok(()),
    }
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
