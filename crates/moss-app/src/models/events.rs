use moss_theme::models::primitives::ThemeId;
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

/// DEPRECATED
/// @category Event
#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "events.ts")]
pub struct ColorThemeChangeEventPayload<'a> {
    #[ts(type = "ThemeId")]
    pub id: &'a ThemeId,
}

impl<'a> ColorThemeChangeEventPayload<'a> {
    pub fn new(id: &'a ThemeId) -> Self {
        Self { id }
    }
}
