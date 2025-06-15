use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::primitives::{LocaleId, ThemeId, ThemeMode};

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
    #[ts(optional, type = "ColorThemeInfo")]
    pub theme: Option<ColorThemeInfo>,

    #[ts(optional, type = "LocaleInfo")]
    pub locale: Option<LocaleInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Defaults {
    #[ts(type = "ColorThemeInfo")]
    pub theme: ColorThemeInfo,

    #[ts(type = "LocaleInfo")]
    pub locale: LocaleInfo,
}
