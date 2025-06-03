use moss_workspace::models::types::WorkspaceMode;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::Path, sync::Arc};
use ts_rs::TS;
use uuid::Uuid;
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
    pub id: Uuid,

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
    pub active_workspace_id: Option<Uuid>,

    #[ts(optional)]
    pub prev_workspace_id: Option<Uuid>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}
