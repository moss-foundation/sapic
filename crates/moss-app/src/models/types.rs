use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;

use crate::models::primitives::{LocaleId, LogLevel, ThemeId, ThemeMode};

// ########################################################
// ###                      Locale                      ###
// ########################################################

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct LocaleInfo {
    pub identifier: LocaleId,
    pub display_name: String,
    pub code: String,
    pub direction: Option<String>,
    pub is_default: Option<bool>,
}

/// @category Type
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ColorThemeInfo {
    pub identifier: ThemeId,
    pub display_name: String,
    pub mode: ThemeMode,
    pub order: Option<usize>,
    pub source: PathBuf,
    pub is_default: Option<bool>,
}

// State

/// @category Type
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct Preferences {
    pub theme: Option<ColorThemeInfo>,
    pub locale: Option<LocaleInfo>,
}

/// @category Type
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Defaults {
    pub theme: ColorThemeInfo,
    pub locale: LocaleInfo,
}

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
    pub id: String,

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
    pub id: String,
    /// A timestamp string, such as "2025-06-06T19:26:39.084+0300"
    pub timestamp: String,
    pub level: LogLevel,
    pub resource: Option<String>,
    pub message: String,
}

// #########################################################
// ###                    Workspace                      ###
// #########################################################

/// @category Type
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct WorkspaceInfo {
    pub id: String,
    pub name: String,
    pub last_opened_at: Option<i64>,
    pub active: bool,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}
