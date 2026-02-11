use std::path::PathBuf;

use sapic_base::{project::types::primitives::ProjectId, resource::types::ResourceSummary};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

//
// List Project Resources
//

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "main/types.ts")]
pub enum ListProjectResourcesMode {
    #[serde(rename = "LOAD_ROOT")]
    LoadRoot,
    #[serde(rename = "RELOAD_PATH")]
    ReloadPath(PathBuf),
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "main/operations.ts")]
pub struct ListProjectResourcesInput {
    pub project_id: ProjectId,
    pub mode: ListProjectResourcesMode,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "main/operations.ts")]
pub struct ListProjectResourcesOutput {
    #[ts(type = "ResourceSummary[]")]
    pub items: Vec<ResourceSummary>,
}
