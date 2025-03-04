use moss_theme::models::types::ThemeDescriptor;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

// #[derive(Debug, Clone, Serialize, Deserialize, TS, Eq, Hash, PartialEq)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "types.ts")]
// pub struct LocaleDescriptor {
//     pub code: String,
//     pub name: String,
//     #[ts(optional)]
//     pub direction: Option<String>,
// }

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Preferences {
    #[ts(optional, type = "ThemeDescriptor")]
    pub theme: Option<ThemeDescriptor>,
    // #[ts(optional)]
    // pub locale: Option<LocaleDescriptor>,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Defaults {
    #[ts(type = "ThemeDescriptor")]
    pub theme: ThemeDescriptor,
    // pub locale: LocaleDescriptor,
}
