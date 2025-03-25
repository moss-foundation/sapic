use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

use crate::primitives::ThemeId;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ThemeMode {
    Light,
    Dark,
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
