use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use ts_rs::TS;

use crate::models::types::LogEntryInfo;

/// @category Event
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename = "OnDidChangeConfiguration")]
#[ts(export, export_to = "events.ts")]
pub struct OnDidChangeConfigurationForFrontend {
    pub affected_keys: Vec<String>,
    #[ts(type = "{ [key: string]: JsonValue }")]
    pub changes: HashMap<String, JsonValue>,
}

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
