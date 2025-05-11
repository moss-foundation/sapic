use moss_common::models::primitives::Identifier;
use moss_workspace::models::types::WorkspaceMode;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::Path, sync::Arc};
use ts_rs::TS;
use validator::Validate;

use crate::models::types::WorkspaceInfo;

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);

impl Deref for ListWorkspacesOutput {
    type Target = Vec<WorkspaceInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Open Workspace

#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct OpenWorkspaceInput {
    /// We use the workspace name instead of its path because
    /// all workspaces can only be stored within a single directory.
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct OpenWorkspaceOutput {
    #[ts(type = "Identifier")]
    pub id: Identifier,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Create Workspace

#[derive(Debug, Validate, Deserialize, TS)]
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
    #[ts(type = "Identifier")]
    pub id: Identifier,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Delete Workspace

#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteWorkspaceInput {
    #[ts(type = "Identifier")]
    pub id: Identifier,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteWorkspaceOutput {
    #[ts(type = "Identifier")]
    pub id: Identifier,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

// Rename Workspace

#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateWorkspaceInput {
    #[ts(type = "Identifier")]
    pub id: Identifier,

    /// A new name for the workspace, if provided,
    /// the workspace will be renamed to this name.
    #[ts(optional)]
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
    pub active_workspace_id: Option<Identifier>,

    #[ts(optional)]
    #[ts(type = "Identifier")]
    pub prev_workspace_id: Option<Identifier>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}
