use moss_environment::models::{
    primitives::EnvironmentId,
    types::{AddVariableParams, VariableInfo},
};
use moss_git::url::GIT_URL_REGEX;
use moss_git_hosting_provider::models::{
    primitives::GitProviderType,
    types::{Contributor, RepositoryInfo},
};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use validator::Validate;

use crate::models::{
    primitives::CollectionId,
    types::{
        EditorPartStateInfo, EnvironmentGroup, UpdateCollectionParams,
        UpdateEnvironmentGroupParams, UpdateEnvironmentParams,
    },
};

use super::types::{
    ActivitybarPartStateInfo, CreateCollectionParams, ImportCollectionParams, PanelPartStateInfo,
    SidebarPartStateInfo,
};

// ------------------------------ //
// Collection
// ------------------------------ //

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateCollectionInput {
    #[serde(flatten)]
    #[validate(nested)]
    pub inner: CreateCollectionParams,
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
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ImportCollectionInput {
    #[serde(flatten)]
    #[validate(nested)]
    pub inner: ImportCollectionParams,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct ImportCollectionOutput {
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
    #[serde(flatten)]
    #[validate(nested)]
    pub inner: UpdateCollectionParams,
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
pub struct BatchUpdateCollectionInput {
    #[validate(nested)]
    pub items: Vec<UpdateCollectionParams>,
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

// FIXME: Should this be refactored to use an inner params?

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
    pub variables: Vec<AddVariableParams>,
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
    pub order: Option<isize>,
    pub color: Option<String>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: PathBuf,
}

// Update Environment

/// @category Operation
#[derive(Debug, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEnvironmentInput {
    #[validate(nested)]
    pub items: Vec<UpdateEnvironmentParams>,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEnvironmentOutput {
    pub ids: Vec<EnvironmentId>,
}

/// @category Operation
#[derive(Debug, Deserialize, Validate, TS)]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEnvironmentInput {
    #[serde(flatten)]
    pub inner: UpdateEnvironmentParams,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEnvironmentOutput {
    pub id: EnvironmentId,
}

// Delete Environment
/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEnvironmentInput {
    pub id: EnvironmentId,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEnvironmentOutput {
    pub id: EnvironmentId,
}

// Stream Environments

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamEnvironmentsOutput {
    pub groups: Vec<EnvironmentGroup>,

    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}

// Describe Collection
/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeCollectionInput {
    pub id: CollectionId,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeCollectionOutput {
    pub name: String,
    pub repository: Option<String>,
    pub repository_info: Option<RepositoryInfo>,
    pub contributors: Vec<Contributor>,
    pub current_branch: Option<String>,
}

// ------------------------------ //
// Environment Group
// ------------------------------ //

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEnvironmentGroupInput {
    #[serde(flatten)]
    pub inner: UpdateEnvironmentGroupParams,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEnvironmentGroupInput {
    #[validate(nested)]
    pub items: Vec<UpdateEnvironmentGroupParams>,
}
