use derive_more::Deref;
use moss_workspace::models::primitives::WorkspaceMode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::types::{primitives::*, *};

//
// List Workspaces
//

/// @category Operation
#[derive(Debug, Serialize, Deref, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);

//
// Open Workspace
//

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

//
// Create Workspace
//

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

//
// Delete Workspace
//

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

//
// Update Workspace
//

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

//
// Close Workspace
//

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
