use moss_bindingutils::primitives::ChangeString;
use moss_environment::models::types::{AddVariableParams, UpdateVariableParams};
use moss_git::models::primitives::FileStatus;
use sapic_base::{
    environment::types::primitives::{EnvironmentId, VariableId},
    project::types::primitives::ProjectId,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use ts_rs::TS;
use validator::Validate;
pub type EnvironmentName = String;

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentGroup {
    pub project_id: Arc<String>,
    pub expanded: bool,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateEnvironmentGroupParams {
    pub project_id: ProjectId,
    pub expanded: Option<bool>,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Debug, Deserialize, Validate, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UpdateEnvironmentParams {
    pub id: EnvironmentId,
    pub name: Option<String>,
    pub order: Option<isize>,
    #[ts(optional, type = "ChangeString")]
    pub color: Option<ChangeString>,
    pub expanded: Option<bool>,
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EntryChange {
    // TODO: entry id
    pub project_id: ProjectId,
    pub path: PathBuf,
    #[ts(type = "FileStatus")]
    pub status: FileStatus,
}
