use derive_more::Deref;
use moss_language::models::types::LanguageInfo;
use moss_logging::models::primitives::LogEntryId;
use sapic_base::{
    user::types::{ProfileInfo, primitives::AccountId},
    workspace::types::WorkspaceInfo,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;
use validator::Validate;

use crate::models::{primitives::*, types::*};

/// DEPRECATED
/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateConfigurationInput {
    #[serde(flatten)]
    pub inner: UpdateConfigurationParams,
}

// #########################################################
// ###                    Profile                        ###
// #########################################################

/// DEPRECATED
/// @category Operation
#[derive(Debug, Clone, Deserialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateProfileInput {
    pub name: String,
    pub is_default: Option<bool>,
}

/// DEPRECATED
/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateProfileOutput {
    pub profile_id: String,
}

/// DEPRECATED
/// @category Operation
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateProfileInput {
    pub accounts_to_add: Vec<AddAccountParams>,
    pub accounts_to_remove: Vec<AccountId>,
    pub accounts_to_update: Vec<UpdateAccountParams>,
}

/// DEPRECATED
/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateProfileOutput {
    pub added_accounts: Vec<AccountId>,
    pub removed_accounts: Vec<AccountId>,
    pub updated_accounts: Vec<AccountId>,
}

// ########################################################
// ###                    Language                      ###
// ########################################################

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationNamespaceInput {
    pub language: String,
    pub namespace: String,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationNamespaceOutput {
    #[ts(type = "JsonValue")]
    pub contents: JsonValue,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListLanguagesOutput(#[ts(type = "LanguageInfo[]")] pub Vec<LanguageInfo>);

// Describe App

/// DEPRECATED
/// @category Operation
#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeAppOutput {
    /// The id of the workspace that is currently opened.
    #[ts(optional, type = "WorkspaceInfo")]
    pub workspace: Option<WorkspaceInfo>,
    /// The id of the profile that is currently active.
    #[ts(optional, type = "ProfileInfo")]
    pub profile: Option<ProfileInfo>,
    pub configuration: Configuration,
}

// #########################################################
// ###                      Log                          ###
// #########################################################

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsInput {
    pub dates: Vec<LogDate>,
    pub levels: Vec<LogLevel>,
    pub resource: Option<String>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsOutput {
    pub contents: Vec<LogEntryInfo>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, Deref, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct BatchDeleteLogInput {
    #[ts(as = "Vec<String>")]
    pub ids: Vec<LogEntryId>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchDeleteLogOutput {
    pub deleted_entries: Vec<LogItemSourceInfo>,
}

// #########################################################
// ###                    Workspace                      ###
// #########################################################

// List Workspaces

// /// @category Operation
// #[derive(Debug, Serialize, Deref, TS)]
// #[ts(export, export_to = "operations.ts")]
// pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);

// // Open Workspace

// /// @category Operation
// #[derive(Debug, Validate, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub struct OpenWorkspaceInput {
//     pub id: WorkspaceId,
// }

// /// DEPRECATED
// /// @category Operation
// #[derive(Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub struct OpenWorkspaceOutput {
//     pub id: WorkspaceId,

//     #[serde(skip)]
//     #[ts(skip)]
//     pub abs_path: Arc<Path>,
// }

// // Create Workspace

// /// @category Operation
// #[derive(Debug, Validate, Deserialize, TS, Clone)]
// #[serde(rename_all = "camelCase")]
// #[ts(optional_fields)]
// #[ts(export, export_to = "operations.ts")]
// pub struct CreateWorkspaceInput {
//     #[validate(length(min = 1))]
//     pub name: String,

//     // FIXME: Do we need this anymore?
//     #[serde(default)]
//     #[ts(type = "WorkspaceMode")]
//     pub mode: WorkspaceMode,

//     #[serde(default = "default_open_on_creation")]
//     pub open_on_creation: bool,
// }

// fn default_open_on_creation() -> bool {
//     true
// }

// /// @category Operation
// #[derive(Debug, Validate, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub struct CreateWorkspaceOutput {
//     pub id: WorkspaceId,

//     pub active: bool,

//     #[serde(skip)]
//     #[ts(skip)]
//     pub abs_path: Arc<Path>,
// }

// // Delete Workspace

// /// @category Operation
// #[derive(Debug, Validate, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub struct DeleteWorkspaceInput {
//     pub id: WorkspaceId,
// }

// /// @category Operation
// #[derive(Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub struct DeleteWorkspaceOutput {
//     pub id: WorkspaceId,

//     #[serde(skip)]
//     #[ts(skip)]
//     pub abs_path: Arc<Path>,
// }

// Rename Workspace

// /// @category Operation
// #[derive(Debug, Validate, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(optional_fields)]
// #[ts(export, export_to = "operations.ts")]
// pub struct UpdateWorkspaceInput {
//     /// A new name for the workspace, if provided, the workspace
//     /// will be renamed to this name.
//     #[validate(length(min = 1))]
//     pub name: Option<String>,
// }

// Describe Workbench State

// /// @category Operation
// #[derive(Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(optional_fields)]
// #[ts(export, export_to = "operations.ts")]
// pub struct DescribeWorkbenchStateOutput {
//     #[serde(skip)]
//     #[ts(skip)]
//     pub active_workspace_id: Option<WorkspaceId>,

//     #[ts(as = "Option<String>")]
//     pub prev_workspace_id: Option<WorkspaceId>,
// }

// // Close Workspace

// /// @category Operation
// #[derive(Debug, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub struct CloseWorkspaceInput {
//     /// The workspace id is required to ensure the close function
//     /// is only called when a workspace is open.
//     pub id: WorkspaceId,
// }

// /// @category Operation
// #[derive(Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub struct CloseWorkspaceOutput {
//     /// The id of the workspace that was closed.
//     pub id: WorkspaceId,
// }
