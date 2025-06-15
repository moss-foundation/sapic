use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;

use crate::models::primitives::ThemeId;

use super::types::{ColorThemeInfo, Defaults, LocaleInfo, Preferences};

// Locale

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationsInput {
    pub language: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationsOutput(#[ts(type = "JsonValue")] pub JsonValue);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListLocalesOutput(pub Vec<LocaleInfo>);

// State

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeAppStateOutput {
    pub preferences: Preferences,
    pub defaults: Defaults,
    #[ts(optional)]
    pub last_workspace: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct SetColorThemeInput {
    #[ts(type = "ColorThemeInfo")]
    pub theme_info: ColorThemeInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct SetLocaleInput {
    #[ts(type = "LocaleInfo")]
    pub locale_info: LocaleInfo,
}

// Theme

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeInput {
    pub id: ThemeId,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeOutput {
    pub css_content: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListColorThemesOutput(pub Vec<ColorThemeInfo>);
