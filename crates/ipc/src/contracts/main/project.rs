use moss_git::models::types::BranchInfo;
use sapic_base::project::types::primitives::ProjectId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

/// @category Event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamProjectsEvent {
    pub id: ProjectId,
    pub name: String,
    pub order: Option<isize>,
    pub expanded: bool,
    pub branch: Option<BranchInfo>,
    pub icon_path: Option<PathBuf>,
    pub archived: bool,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamProjectsOutput {
    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}
