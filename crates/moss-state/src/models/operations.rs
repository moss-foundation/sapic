use moss_nls::models::types::LocaleInfo;
use moss_theme::models::types::ColorThemeInfo;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::types::{Defaults, Preferences};

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeAppStateOutput {
    pub preferences: Preferences,
    pub defaults: Defaults,
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
