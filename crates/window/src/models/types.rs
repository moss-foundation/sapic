use moss_logging::models::primitives::LogEntryId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

use crate::models::primitives::*;

// #########################################################
// ###                      Log                          ###
// #########################################################

// FIXME: Is this the best way to handle date type?
/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct LogDate {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct LogItemSourceInfo {
    pub id: LogEntryId,

    #[serde(skip)]
    /// None if deleted from in-memory queue
    pub file_path: Option<PathBuf>,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct LogEntryInfo {
    pub id: LogEntryId,
    /// A timestamp string, such as "2025-06-06T19:26:39.084+0300"
    pub timestamp: String,
    pub level: LogLevel,
    pub resource: Option<String>,
    pub message: String,
}
