use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

use crate::models::types::{LogDate, LogEntryInfo, LogLevel};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsInput {
    pub dates: Vec<LogDate>,
    pub levels: Vec<LogLevel>,
    #[ts(optional)]
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsOutput {
    pub contents: Vec<LogEntryInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteLogInput {
    pub timestamp: String, // This helps us to narrow down the scope of search
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteLogOutput {
    pub id: String,
    #[serde(skip)]
    // None if deleted from in-memory queue
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteLogsInput {
    pub entries: Vec<DeleteLogInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteLogsOutput {
    pub deleted_entries: Vec<DeleteLogOutput>,
}
