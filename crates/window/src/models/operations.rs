use derive_more::Deref;
use moss_logging::models::primitives::LogEntryId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{primitives::*, types::*};

// #########################################################
// ###                      Log                          ###
// #########################################################

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsInput {
    pub dates: Vec<LogDate>,
    pub levels: Vec<LogLevel>,
    pub resource: Option<String>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsOutput {
    pub contents: Vec<LogEntryInfo>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, Deref, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct BatchDeleteLogInput {
    #[ts(as = "Vec<String>")]
    pub ids: Vec<LogEntryId>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchDeleteLogOutput {
    pub deleted_entries: Vec<LogItemSourceInfo>,
}
