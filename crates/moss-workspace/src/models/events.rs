use std::path::PathBuf;

use moss_git::models::types::BranchInfo;
use sapic_base::{
    environment::types::primitives::EnvironmentId, project::types::primitives::ProjectId,
};
use serde::{Deserialize, Serialize};
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

/// @category Event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEnvironmentsEvent {
    pub id: EnvironmentId,

    /// The id of the project that the environment belongs to.
    /// If the environment is global, this will be `None`.
    pub project_id: Option<ProjectId>,
    pub is_active: bool,

    pub name: String,
    pub order: Option<isize>,
    pub total_variables: usize,
}
