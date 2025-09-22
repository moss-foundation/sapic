use moss_configuration::models::primitives::ParameterType;
use moss_logging::models::primitives::LogEntryId;
use moss_user::models::primitives::AccountKind;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;

use crate::models::primitives::*;

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ConfigurationSchema {
    pub id: String,
    pub parent_id: Option<String>,
    pub order: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<ConfigurationParameterValue>,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ConfigurationParameterValue {
    pub id: String,
    #[ts(optional, type = "JsonValue")]
    pub default: Option<JsonValue>,
    #[ts(type = "ParameterType")]
    pub typ: ParameterType,
    pub description: Option<String>,
    pub maximum: Option<u64>,
    pub minimum: Option<u64>,
    pub protected: bool,
    pub order: Option<i64>,
    pub tags: Vec<String>,
}

/// @category Type
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateConfigurationParams {
    pub key: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub target: ConfigurationTarget,
}

/// @category Type
#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct Configuration {
    pub keys: Vec<String>,
    #[ts(type = "{ [key: string]: JsonValue }")]
    pub contents: HashMap<String, JsonValue>,
}

// ########################################################
// ###                      Profile                     ###
// ########################################################

/// @category Type
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct AddAccountParams {
    pub host: String,
    pub label: Option<String>,
    pub kind: AccountKind,
    /// If a PAT is not provided, we will use OAuth
    pub pat: Option<String>,
}

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
    pub order: Option<isize>,
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
    pub order: Option<isize>,
    pub source: PathBuf,
    pub is_default: Option<bool>,
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
    #[ts(as = "String")]
    pub id: LogEntryId,

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
    #[ts(as = "String")]
    pub id: LogEntryId,
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
    pub id: WorkspaceId,
    pub name: String,
    pub last_opened_at: Option<i64>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}
