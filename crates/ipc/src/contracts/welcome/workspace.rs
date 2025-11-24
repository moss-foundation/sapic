use sapic_base::workspace::types::primitives::WorkspaceId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

//
// Create Workspace
//

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS, Clone)]
#[serde(rename = "WelcomeWindow_CreateWorkspaceInput")]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "welcome/operations.ts")]
pub struct CreateWorkspaceInput {
    #[validate(length(min = 1))]
    pub name: String,
}

/// @category Operation
#[derive(Debug, Validate, Serialize, TS, Clone)]
#[serde(rename = "WelcomeWindow_CreateWorkspaceOutput")]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "welcome/operations.ts")]
pub struct CreateWorkspaceOutput {
    pub id: WorkspaceId,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: PathBuf,
}

//
// Open Workspace
//

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "WelcomeWindow_OpenWorkspaceInput")]
#[ts(export, export_to = "welcome/operations.ts")]
pub struct OpenWorkspaceInput {
    pub id: WorkspaceId,
}

/// DEPRECATED
/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "welcome/operations.ts")]
pub struct OpenWorkspaceOutput {
    pub id: WorkspaceId,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: PathBuf,
}

//
// Update Workspace
//

/// @category Operation
///
/// Used for updating any workspace from any window.
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename = "WelcomeWindow_UpdateWorkspaceInput")]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "welcome/operations.ts")]
pub struct UpdateWorkspaceInput {
    pub id: WorkspaceId,

    /// A new name for the workspace, if provided, the workspace
    /// will be renamed to this name.
    #[validate(length(min = 1))]
    pub name: Option<String>,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename = "WelcomeWindow_UpdateWorkspaceOutput")]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "welcome/operations.ts")]
pub struct UpdateWorkspaceOutput {}
