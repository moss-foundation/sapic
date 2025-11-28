use derive_more::Deref;
use sapic_base::workspace::types::{WorkspaceInfo, primitives::WorkspaceId};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

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
    pub abs_path: Option<PathBuf>,
}

//
// List Workspaces
//

/// @category Operation
#[derive(Debug, Serialize, Deref, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListWorkspacesOutput(pub Vec<WorkspaceInfo>);
