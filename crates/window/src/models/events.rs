use serde::Serialize;
use ts_rs::TS;

use crate::models::types::LogEntryInfo;

/// @category Event
#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename = "OnDidAppendLogEntry")]
#[ts(export, export_to = "events.ts")]
pub struct OnDidAppendLogEntryForFrontend {
    #[serde(flatten)]
    pub inner: LogEntryInfo,
}

/// @category Event
#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename = "OnDidAddExtension")]
#[ts(export, export_to = "events.ts")]
pub struct OnDidAddExtensionForFrontend {
    pub id: String,
}
