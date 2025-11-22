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
#[ts(export, export_to = "welcome/workspace.ts")]
pub struct CreateWorkspaceInput {
    #[validate(length(min = 1))]
    pub name: String,
}

/// @category Operation
#[derive(Debug, Validate, Serialize, TS, Clone)]
#[serde(rename = "WelcomeWindow_CreateWorkspaceOutput")]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "welcome/workspace.ts")]
pub struct CreateWorkspaceOutput {
    pub id: WorkspaceId,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: PathBuf,
}
