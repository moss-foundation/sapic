use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::types::*;

// Get File Statuses
/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListChangesOutput {
    pub changes: Vec<EntryChange>,
}
