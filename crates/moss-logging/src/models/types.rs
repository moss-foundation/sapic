use serde::{Deserialize, Serialize};
use ts_rs::TS;

// FIXME: Is this the best way to handle date type?
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct LogDate {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "types.ts")]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct LogEntry {
    timestamp: String,
    level: String,
    #[ts(optional)]
    request: Option<String>,
    #[ts(optional)]
    collection: Option<String>,
    message: String,
}
