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
