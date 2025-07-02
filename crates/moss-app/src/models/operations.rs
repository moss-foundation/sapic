use std::{path::Path, sync::Arc};

use derive_more::Deref;
use moss_workspace::models::types::WorkspaceMode;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;
use uuid::Uuid;
use validator::Validate;

use crate::models::{
    primitives::{LogLevel, ThemeId},
    types::{LogDate, LogEntryInfo, LogItemSourceInfo, WorkspaceInfo},
};

use super::types::{ColorThemeInfo, Defaults, LocaleInfo, Preferences};

// ########################################################
// ###                      Locale                      ###
// ########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationsInput {
    pub language: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationsOutput(#[ts(type = "JsonValue")] pub JsonValue);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListLocalesOutput(pub Vec<LocaleInfo>);

// Describe App State

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeAppStateOutput {
    pub preferences: Preferences,
    pub defaults: Defaults,
    pub last_workspace: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct SetColorThemeInput {
    pub theme_info: ColorThemeInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct SetLocaleInput {
    pub locale_info: LocaleInfo,
}

// ########################################################
// ###                      Theme                       ###
// ########################################################

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeInput {
    pub id: ThemeId,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeOutput {
    pub css_content: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListColorThemesOutput(pub Vec<ColorThemeInfo>);

// #########################################################
// ###                      Log                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsInput {
    pub dates: Vec<LogDate>,
    pub levels: Vec<LogLevel>,
    #[ts(optional)]
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsOutput {
    pub contents: Vec<LogEntryInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct BatchDeleteLogInput(pub Vec<String>);

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

#[derive(Debug, Deserialize, Serialize, Deref, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);

// Open Workspace

#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct OpenWorkspaceInput {
    pub id: Uuid,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct OpenWorkspaceOutput {
    pub id: Uuid,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Create Workspace

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

#[derive(Debug, Validate, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateWorkspaceOutput {
    pub id: Uuid,

    pub active: bool,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Delete Workspace

#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteWorkspaceInput {
    pub id: Uuid,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteWorkspaceOutput {
    pub id: Uuid,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Rename Workspace

#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateWorkspaceInput {
    /// A new name for the workspace, if provided, the workspace
    /// will be renamed to this name.
    #[validate(length(min = 1))]
    pub name: Option<String>,
}

// Describe Workbench State

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeWorkbenchStateOutput {
    #[serde(skip)]
    #[ts(skip)]
    pub active_workspace_id: Option<Uuid>,

    pub prev_workspace_id: Option<Uuid>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Close Workspace

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CloseWorkspaceInput {
    /// The workspace id is required to ensure the close function
    /// is only called when a workspace is open.
    pub id: Uuid,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CloseWorkspaceOutput {
    /// The id of the workspace that was closed.
    pub id: Uuid,
}
