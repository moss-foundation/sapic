use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::primitives::{LocaleId, LogLevel, ThemeId, ThemeMode};

// Locale

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct LocaleInfo {
    pub identifier: LocaleId,
    pub display_name: String,
    pub code: String,
    #[ts(optional)]
    pub direction: Option<String>,
    #[ts(optional)]
    pub is_default: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ColorThemeInfo {
    pub identifier: ThemeId,
    pub display_name: String,
    pub mode: ThemeMode,
    #[ts(optional)]
    pub order: Option<usize>,
    pub source: PathBuf,
    #[ts(optional)]
    pub is_default: Option<bool>,
}

// State

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Preferences {
    pub theme: Option<ColorThemeInfo>,
    pub locale: Option<LocaleInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Defaults {
    pub theme: ColorThemeInfo,
    pub locale: LocaleInfo,
}

// Log

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct LogEntryRef {
    pub id: String,
}

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
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct LogItemSourceInfo {
    pub id: String,
    #[serde(skip)]
    /// None if deleted from in-memory queue
    pub file_path: Option<PathBuf>,
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
