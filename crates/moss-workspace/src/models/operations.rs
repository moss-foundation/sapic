use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_environment::models::types::VariableInfo;
use moss_git::url::GIT_URL_REGEX;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::models::{primitives::CollectionId, types::EditorPartStateInfo};

use super::types::{ActivitybarPartStateInfo, PanelPartStateInfo, SidebarPartStateInfo};

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionInput {
    #[validate(length(min = 1))]
    pub name: String,

    pub order: usize,
    pub external_path: Option<PathBuf>,
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repo: Option<String>,
    pub icon_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionOutput {
    #[ts(type = "string")]
    pub id: CollectionId,
    pub name: String,
    pub order: Option<usize>,
    pub expanded: bool,
    pub icon_path: Option<PathBuf>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,

    #[serde(skip)]
    #[ts(skip)]
    pub external_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateCollectionInput {
    #[ts(type = "string")]
    pub id: CollectionId,

    #[validate(length(min = 1))]
    pub name: Option<String>,

    #[validate(custom(function = "validate_change_repository"))]
    pub repository: Option<ChangeString>,

    // TODO: add validation
    pub icon_path: Option<ChangePath>,
    pub order: Option<usize>,
    pub pinned: Option<bool>,
    pub expanded: Option<bool>,
}

fn validate_change_repository(repo: &ChangeString) -> Result<(), ValidationError> {
    match repo {
        ChangeString::Update(repo) => GIT_URL_REGEX
            .is_match(repo)
            .then_some(())
            .ok_or(ValidationError::new("Invalid Git URL format")),
        ChangeString::Remove => Ok(()),
    }
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateCollectionOutput {
    #[ts(type = "string")]
    pub id: CollectionId,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteCollectionInput {
    #[ts(type = "string")]
    pub id: CollectionId,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteCollectionOutput {
    #[ts(type = "string")]
    pub id: CollectionId,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Option<Arc<Path>>,
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentInput {
    #[ts(type = "string")]
    pub id: CollectionId,
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
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeStateOutput {
    pub editor: Option<EditorPartStateInfo>,
    pub sidebar: Option<SidebarPartStateInfo>,
    pub panel: Option<PanelPartStateInfo>,
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

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamCollectionsOutput {
    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}
