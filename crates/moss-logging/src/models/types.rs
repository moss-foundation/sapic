use serde::{Deserialize, Serialize};
use tracing::Level;
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

impl Into<Level> for LogLevel {
    fn into(self) -> Level {
        match self {
            LogLevel::TRACE => Level::TRACE,
            LogLevel::DEBUG => Level::DEBUG,
            LogLevel::INFO => Level::INFO,
            LogLevel::WARN => Level::WARN,
            LogLevel::ERROR => Level::ERROR,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct LogEntryInfo {
    pub id: String,
    /// A timestamp string, such as "2025-06-06T19:26:39.084+0300"
    pub timestamp: String,
    pub level: LogLevel,
    #[ts(optional)]
    pub resource: Option<String>,
    pub message: String,
}
