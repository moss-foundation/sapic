use moss_nls::models::types::LocaleInfo;
use moss_theme::models::types::ColorThemeInfo;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

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
