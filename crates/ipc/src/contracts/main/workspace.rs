use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

//
// Update Workspace
//

/// @category Operation
///
/// Used only for the update operation from the main window, specifically to modify the active workspace.
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

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateWorkspaceOutput {}
