use moss_environment::models::types::AddVariableParams;
use sapic_base::{
    environment::types::{VariableInfo, primitives::EnvironmentId},
    project::types::primitives::ProjectId,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

use crate::models::types::*;

// Get File Statuses
/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListChangesOutput {
    pub changes: Vec<EntryChange>,
}
