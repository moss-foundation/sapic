use crate::models::types::{LogDate, LogEntry, LogLevel};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsInput {
    pub dates: Vec<LogDate>,
    pub levels: Vec<LogLevel>,
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsOutput {
    pub contents: Vec<LogEntry>,
}
