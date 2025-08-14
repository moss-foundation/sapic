use moss_environment::models::{
    primitives::EnvironmentId,
    types::{AddVariableParams, VariableInfo},
};
use moss_git::url::GIT_URL_REGEX;
use moss_git_hosting_provider::models::primitives::GitProviderType;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use validator::Validate;

use crate::models::{
    primitives::CollectionId,
    types::{EditorPartStateInfo, UpdateCollectionParams, UpdateEnvironmentParams},
};

use super::types::{
    ActivitybarPartStateInfo, GitHubImportParams, GitLabImportParams, PanelPartStateInfo,
    SidebarPartStateInfo,
};

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

    // FIXME: Pass also the git provider information
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: Option<String>,

    // FIXME: Replace the repo input type with an enum
    #[serde(skip)]
    #[ts(skip)]
    pub git_provider_type: Option<GitProviderType>,

    // TODO: repo branch
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
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum ImportCollectionInput {
    GitHub(GitHubImportParams),
    GitLab(GitLabImportParams),
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
    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}
