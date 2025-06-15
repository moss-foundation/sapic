use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type LocaleId = String;
pub type ThemeId = String;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ThemeMode {
    Light,
    Dark,
}
