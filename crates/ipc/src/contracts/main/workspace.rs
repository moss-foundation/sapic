use sapic_base::workspace::types::primitives::WorkspaceId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

use crate::contracts::main::OpenInTarget;

//
// Create Workspace
//

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS, Clone)]
#[serde(rename = "MainWindow_CreateWorkspaceInput")]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "main/operations.ts")]
pub struct CreateWorkspaceInput {
    #[validate(length(min = 1))]
    pub name: String,

    #[serde(default = "default_open_on_creation")]
    pub open_on_creation: OpenInTarget,
}

fn default_open_on_creation() -> OpenInTarget {
    OpenInTarget::NewWindow
}

/// @category Operation
#[derive(Debug, Validate, Serialize, TS, Clone)]
#[serde(rename = "MainWindow_CreateWorkspaceOutput")]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "main/operations.ts")]
pub struct CreateWorkspaceOutput {
    pub id: WorkspaceId,

    /// Whether the workspace was replaced by the new one.
    /// This is only true if the workspace was created in the same window.
    ///
    /// Technically, this field doesn't carry any functionality and remains for
    /// backward compatibility, since now the frontend can determine whether to
    /// replace the workspace based on the `open_on_creation` parameter.
    pub will_replace: bool,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: PathBuf,
}

//
// Update Workspace
//

/// @category Operation
///
/// Used only for the update operation from the main window, specifically to modify the active workspace.
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename = "MainWindow_UpdateWorkspaceInput")]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "main/operations.ts")]
pub struct UpdateWorkspaceInput {
    /// A new name for the workspace, if provided, the workspace
    /// will be renamed to this name.
    #[validate(length(min = 1))]
    pub name: Option<String>,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename = "MainWindow_UpdateWorkspaceOutput")]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "main/operations.ts")]
pub struct UpdateWorkspaceOutput {}

//
// Open Workspace
//

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename = "MainWindow_OpenWorkspaceInput")]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "main/operations.ts")]
pub struct OpenWorkspaceInput {
    pub id: WorkspaceId,

    #[serde(default = "default_open_in_target")]
    pub open_in_target: OpenInTarget,
}

fn default_open_in_target() -> OpenInTarget {
    OpenInTarget::CurrentWindow
}
