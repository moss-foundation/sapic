use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_environment::models::{
    primitives::{EnvironmentId, VariableId},
    types::{AddVariableParams, UpdateVariableParams, VariableInfo},
};
use moss_git::url::GIT_URL_REGEX;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::models::{
    primitives::{ChangeCollectionId, CollectionId},
    types::EditorPartStateInfo,
};

use super::types::{ActivitybarPartStateInfo, PanelPartStateInfo, SidebarPartStateInfo};

// ------------------------------ //
// Collection
// ------------------------------ //

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionInput {
    #[validate(length(min = 1))]
    pub name: String,

    pub order: isize,
    pub external_path: Option<PathBuf>,
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: Option<String>,
    pub icon_path: Option<PathBuf>,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionOutput {
    pub id: CollectionId,
    pub name: String,
    pub order: Option<isize>,
    pub expanded: bool,
    pub icon_path: Option<PathBuf>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,

    #[serde(skip)]
    #[ts(skip)]
    pub external_path: Option<PathBuf>,
}

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateCollectionInput {
    pub id: CollectionId,

    #[validate(length(min = 1))]
    pub name: Option<String>,

    #[validate(custom(function = "validate_change_repository"))]
    #[ts(optional, type = "ChangeString")]
    pub repository: Option<ChangeString>,

    // TODO: add validation
    #[ts(optional, type = "ChangePath")]
    pub icon_path: Option<ChangePath>,
    pub order: Option<isize>,
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

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateCollectionOutput {
    #[ts(type = "string")]
    pub id: CollectionId,
}

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateCollectionParams {
    #[ts(type = "string")]
    pub id: CollectionId,

    pub order: Option<isize>,
    pub expanded: Option<bool>,
}

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateCollectionInput {
    pub items: Vec<BatchUpdateCollectionParams>,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateCollectionOutput {
    #[ts(as = "Vec<String>")]
    pub ids: Vec<CollectionId>,
}

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteCollectionInput {
    #[ts(type = "string")]
    pub id: CollectionId,
}

/// @category Operation
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

/// @category Operation
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentInput {
    #[ts(type = "string")]
    pub id: CollectionId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentOutput {
    #[ts(type = "VariableInfo")]
    pub variables: Vec<VariableInfo>,
}

/// @category Operation
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

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateStateInput {
    UpdateEditorPartState(EditorPartStateInfo),
    UpdateSidebarPartState(SidebarPartStateInfo),
    UpdatePanelPartState(PanelPartStateInfo),
    UpdateActivitybarPartState(ActivitybarPartStateInfo),
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamCollectionsOutput {
    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}

// ------------------------------ //
// Environment
// ------------------------------ //

// Create Environment

/// @category Operation
#[derive(Debug, Deserialize, Serialize, Validate, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateEnvironmentInput {
    pub collection_id: Option<CollectionId>,
    #[validate(length(min = 1))]
    pub name: String,
    pub order: isize,
    pub color: Option<String>,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateEnvironmentOutput {
    pub id: EnvironmentId,
    pub collection_id: Option<CollectionId>,
    pub name: String,
    pub order: isize,
    pub color: Option<String>,
    pub expanded: bool,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: PathBuf,
}

// Update Environment

/// @category Operation
#[derive(Debug, Deserialize, Validate, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEnvironmentInput {
    pub id: EnvironmentId,

    /// When updating an environment, we can move it to another collection
    /// or remove its link to a specific collection to make it global.
    pub collection_id: Option<ChangeCollectionId>,
    pub name: Option<String>,
    pub order: Option<isize>,
    #[ts(optional, type = "ChangeString")]
    pub color: Option<ChangeString>,
    pub expanded: Option<bool>,
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEnvironmentOutput {
    pub id: EnvironmentId,
}

// Stream Environments

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamEnvironmentsOutput {
    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}
