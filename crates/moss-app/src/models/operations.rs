use derive_more::Deref;
use moss_configuration::models::types::ConfigurationSchema;
use moss_language::models::primitives::{LanguageCode, LanguageDirection};
use moss_logging::models::primitives::LogEntryId;
use moss_theme::models::primitives::ThemeId;
use moss_user::models::{primitives::AccountId, types::ProfileInfo};
use moss_workspace::models::primitives::WorkspaceMode;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};
use ts_rs::TS;
use validator::Validate;

use crate::models::{primitives::*, types::*};

/// @category Operation
#[derive(Debug, Clone, Serialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListConfigurationSchemasOutput {
    #[ts(type = "ConfigurationSchema[]")]
    pub schemas: Vec<ConfigurationSchema>,
}

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

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateProfileInput {
    pub name: String,
    pub is_default: Option<bool>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateProfileOutput {
    pub profile_id: String,
}

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateProfileInput {
    pub accounts_to_add: Vec<AddAccountParams>,
    pub accounts_to_remove: Vec<AccountId>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateProfileOutput {
    pub added_accounts: Vec<AccountId>,
    pub removed_accounts: Vec<AccountId>,
}

// ########################################################
// ###                   Cancellation                   ###
// ########################################################

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct CancelRequestInput {
    pub request_id: String,
}

// ########################################################
// ###                      Locale                      ###
// ########################################################

/// DEPRECATED
/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetLocaleInput {
    pub identifier: String,
}

/// DEPRECATED
/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct GetLocaleOutput {
    pub display_name: String,
    pub code: LanguageCode,
    #[ts(optional, type = "LanguageDirection")]
    pub direction: Option<LanguageDirection>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeLocaleOutput {
    pub display_name: String,
    pub code: String,
    #[ts(optional, type = "LanguageDirection")]
    pub direction: Option<LanguageDirection>,
}

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
pub struct ListLocalesOutput(pub Vec<LocaleInfo>);

// Describe App

/// @category Operation
#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeAppOutput {
    /// The id of the workspace that is currently opened.
    pub workspace: Option<WorkspaceInfo>,
    /// The id of the profile that is currently active.
    #[ts(optional, type = "ProfileInfo")]
    pub profile: Option<ProfileInfo>,
    pub configuration: Configuration,
}

// DEPRECATED
/// @category Operation
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct SetLocaleInput {
    pub locale_info: LocaleInfo,
}

// ########################################################
// ###                      Theme                       ###
// ########################################################

/// @category Operation
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeInput {
    #[ts(type = "ThemeId")]
    pub id: ThemeId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeOutput {
    pub css_content: String,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListColorThemesOutput(pub Vec<ColorThemeInfo>);

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

/// @category Operation
#[derive(Debug, Serialize, Deref, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);

// Open Workspace

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct OpenWorkspaceInput {
    pub id: WorkspaceId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct OpenWorkspaceOutput {
    pub id: WorkspaceId,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Create Workspace

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateWorkspaceInput {
    #[validate(length(min = 1))]
    pub name: String,

    #[serde(default)]
    #[ts(type = "WorkspaceMode")]
    pub mode: WorkspaceMode,

    #[serde(default = "default_open_on_creation")]
    pub open_on_creation: bool,
}

fn default_open_on_creation() -> bool {
    true
}

/// @category Operation
#[derive(Debug, Validate, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateWorkspaceOutput {
    pub id: WorkspaceId,

    pub active: bool,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Delete Workspace

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteWorkspaceInput {
    pub id: WorkspaceId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteWorkspaceOutput {
    pub id: WorkspaceId,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Rename Workspace

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateWorkspaceInput {
    /// A new name for the workspace, if provided, the workspace
    /// will be renamed to this name.
    #[validate(length(min = 1))]
    pub name: Option<String>,
}

// Describe Workbench State

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeWorkbenchStateOutput {
    #[serde(skip)]
    #[ts(skip)]
    pub active_workspace_id: Option<WorkspaceId>,

    #[ts(as = "Option<String>")]
    pub prev_workspace_id: Option<WorkspaceId>,
}

// Close Workspace

/// @category Operation
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CloseWorkspaceInput {
    /// The workspace id is required to ensure the close function
    /// is only called when a workspace is open.
    pub id: WorkspaceId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CloseWorkspaceOutput {
    /// The id of the workspace that was closed.
    pub id: WorkspaceId,
}
