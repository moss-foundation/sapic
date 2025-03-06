use moss_nls::models::types::LocaleDescriptor;
use moss_theme::models::types::ThemeDescriptor;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Preferences {
    #[ts(optional, type = "ThemeDescriptor")]
    pub theme: Option<ThemeDescriptor>,

    #[ts(optional, type = "LocaleDescriptor")]
    pub locale: Option<LocaleDescriptor>,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Defaults {
    #[ts(type = "ThemeDescriptor")]
    pub theme: ThemeDescriptor,

    #[ts(type = "LocaleDescriptor")]
    pub locale: LocaleDescriptor,
}
