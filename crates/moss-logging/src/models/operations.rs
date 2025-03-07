use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::models::primitives::{CollectionPath, RequestPath};
use crate::models::types::{LogDate, LogEntry, LogLevel};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsInput {
    pub dates: Vec<LogDate>,
    pub levels: Vec<LogLevel>,
    #[ts(optional)]
    pub collection: Option<CollectionPath>,
    #[ts(optional)]
    pub request: Option<RequestPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLogsOutput {
    pub contents: Vec<LogEntry>,
}