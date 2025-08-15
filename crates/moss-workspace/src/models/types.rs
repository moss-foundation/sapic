mod editor;
pub use editor::*;

use moss_environment::models::types::VariableInfo;
use moss_git::url::GIT_URL_REGEX;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;
use validator::Validate;

use super::primitives::{ActivitybarPosition, SidebarPosition};

pub type EnvironmentName = String;

/// @category Type
#[derive(Debug, PartialEq, Serialize, Deserialize, TS, Clone)]
#[ts(export, export_to = "types.ts")]
pub enum WorkspaceMode {
    #[serde(rename = "DESIGN_FIRST")]
    DesignFirst,

    #[serde(rename = "REQUEST_FIRST")]
    RequestFirst,
}

impl Default for WorkspaceMode {
    fn default() -> Self {
        Self::RequestFirst
    }
}

/// @category Type
#[derive(Debug, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CollectionInfo {
    pub id: String,
    pub display_name: String,
    pub order: Option<isize>,
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
    pub size: usize,
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
    pub size: usize,
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

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum ImportCollectionParams {
    GitHub(GitHubImportParams),
    GitLab(GitLabImportParams),
}

// FIXME: Validation for provider specific url?
/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct GitHubImportParams {
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// If provided, this branch will be checked out instead of the default branch
    pub branch: Option<String>,
}

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct GitLabImportParams {
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// If provided, this branch will be checked out instead of the default branch
    pub branch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum CreateCollectionGitParams {
    GitHub(GitHubCreateParams),
    GitLab(GitLabCreateParams),
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GitHubCreateParams {
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// The name of the default branch
    pub branch: String,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GitLabCreateParams {
    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// The name of the default branch
    pub branch: String,
}
