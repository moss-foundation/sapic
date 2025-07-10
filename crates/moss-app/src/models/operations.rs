use std::{path::Path, sync::Arc};

use derive_more::Deref;
use moss_workspace::models::types::WorkspaceMode;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;
use validator::Validate;

use super::types::{ColorThemeInfo, Defaults, LocaleInfo, Preferences};
use crate::models::{
    primitives::{LogEntryId, LogLevel, ThemeId, WorkspaceId},
    types::{LogDate, LogEntryInfo, LogItemSourceInfo, WorkspaceInfo},
};
// ########################################################
// ###                      Locale                      ###
// ########################################################

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationsInput {
    pub language: String,
    pub namespace: String,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationsOutput(#[ts(type = "JsonValue")] pub JsonValue);

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListLocalesOutput(pub Vec<LocaleInfo>);

// Describe App State

/// @category Operation
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeAppStateOutput {
    pub preferences: Preferences,
    pub defaults: Defaults,
    #[ts(as = "Option<String>")]
    pub prev_workspace_id: Option<WorkspaceId>,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct SetColorThemeInput {
    pub theme_info: ColorThemeInfo,
}

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
    #[ts(as = "String")]
    pub id: WorkspaceId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct OpenWorkspaceOutput {
    #[ts(as = "String")]
    pub id: WorkspaceId,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Create Workspace

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
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
    #[ts(as = "String")]
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
    #[ts(as = "String")]
    pub id: WorkspaceId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteWorkspaceOutput {
    #[ts(as = "String")]
    pub id: WorkspaceId,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Rename Workspace

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts", optional_fields)]
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
#[ts(export, export_to = "operations.ts", optional_fields)]
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
    #[ts(as = "String")]
    pub id: WorkspaceId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CloseWorkspaceOutput {
    /// The id of the workspace that was closed.
    #[ts(as = "String")]
    pub id: WorkspaceId,
}
