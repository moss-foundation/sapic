mod editor;
pub use editor::*;

use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_environment::models::{
    primitives::{EnvironmentId, VariableId},
    types::{AddVariableParams, UpdateVariableParams, VariableInfo},
};
use moss_git::{models::types::BranchInfo, repository::FileStatus, url::GIT_URL_REGEX};
use moss_user::models::primitives::AccountId;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::models::primitives::{ActivitybarPosition, ProjectId, SidebarPosition};

pub type EnvironmentName = String;

// ------------------------------ //
// Collection
// ------------------------------ //

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CreateCollectionParams {
    #[validate(length(min = 1))]
    pub name: String,

    pub order: isize,
    pub external_path: Option<PathBuf>,

    pub git_params: Option<CreateCollectionGitParams>,

    pub icon_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ImportCollectionParams {
    #[validate(length(min = 1))]
    pub name: String,
    pub order: isize,
    pub external_path: Option<PathBuf>,
    pub source: ImportCollectionSource,
    pub icon_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ExportCollectionParams {
    pub id: ProjectId,
    /// Path to the folder containing the output archive file
    #[validate(custom(function = "validate_export_destination"))]
    pub destination: PathBuf,
}

fn validate_export_destination(destination: &Path) -> Result<(), ValidationError> {
    if !destination.is_dir() {
        return Err(ValidationError::new("destination must be a directory"));
    }
    Ok(())
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentGroup {
    pub collection_id: Arc<String>,
    pub expanded: bool,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateEnvironmentGroupParams {
    pub collection_id: ProjectId,
    pub expanded: Option<bool>,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateCollectionParams {
    pub id: ProjectId,

    #[validate(length(min = 1))]
    pub name: Option<String>,

    #[validate(custom(function = "validate_change_repository"))]
    #[ts(optional, type = "ChangeString")]
    pub repository: Option<ChangeString>,

    // TODO: add validation
    #[ts(optional, type = "ChangePath")]
    pub icon_path: Option<ChangePath>,
    pub order: Option<isize>,
    pub expanded: Option<bool>,
}

/// @category Type
#[derive(Debug, Deserialize, Validate, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UpdateEnvironmentParams {
    pub id: EnvironmentId,
    pub name: Option<String>,
    pub order: Option<isize>,
    #[ts(optional, type = "ChangeString")]
    pub color: Option<ChangeString>,
    pub expanded: Option<bool>,
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
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

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfo {
    pub id: String,
    pub collection_id: Option<String>,
    pub name: String,
    pub display_name: String,
    pub order: isize,
    pub color: Option<String>,
    pub variables: Vec<VariableInfo>,
}

// ------------------------------------------------------------
// Activitybar Part State
// ------------------------------------------------------------

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ActivitybarItemStateInfo {
    pub id: String,
    pub order: isize,
    pub visible: bool,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ActivitybarPartStateInfo {
    pub last_active_container_id: Option<String>,
    pub position: ActivitybarPosition,
    pub items: Vec<ActivitybarItemStateInfo>,
}

// ------------------------------------------------------------
// Sidebar Part State
// ------------------------------------------------------------

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct SidebarPartStateInfo {
    pub position: SidebarPosition,
    pub size: f64,
    pub visible: bool,
}

// ------------------------------------------------------------
// Panel Part State
// ------------------------------------------------------------

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PanelPartStateInfo {
    pub size: f64,
    pub visible: bool,
}

// ------------------------------------------------------------
// Editor Part State
// ------------------------------------------------------------

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EditorPartStateInfo {
    pub grid: EditorGridState,
    #[ts(type = "Record<string, EditorPanelState>")]
    pub panels: HashMap<String, EditorPanelState>,
    pub active_group: Option<String>,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ImportCollectionSource {
    GitHub(GitHubImportParams),
    GitLab(GitLabImportParams),
    Archive(ArchiveImportParams),
}

// FIXME: Validation for provider specific url?
/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct GitHubImportParams {
    pub account_id: AccountId,

    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// If provided, this branch will be checked out instead of the default branch
    pub branch: Option<String>,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct GitLabImportParams {
    pub account_id: AccountId,

    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// If provided, this branch will be checked out instead of the default branch
    pub branch: Option<String>,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ArchiveImportParams {
    pub archive_path: PathBuf,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum CreateCollectionGitParams {
    GitHub(GitHubCreateParams),
    GitLab(GitLabCreateParams),
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct GitHubCreateParams {
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// The name of the default branch
    pub branch: String,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct GitLabCreateParams {
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// The name of the default branch
    pub branch: String,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum VcsInfo {
    GitHub(GitHubVcsInfo),
    GitLab(GitLabVcsInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct GitHubVcsInfo {
    pub branch: BranchInfo,
    pub url: String,
    pub updated_at: Option<String>,
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct GitLabVcsInfo {
    pub branch: BranchInfo,
    pub url: String,
    pub updated_at: Option<String>,
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct Contributor {
    pub name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EntryChange {
    // TODO: entry id
    pub collection_id: ProjectId,
    pub path: PathBuf,
    #[ts(type = "FileStatus")]
    pub status: FileStatus,
}
