pub mod primitives;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

use crate::theme::types::primitives::{ThemeId, ThemeMode};

/// @category Type
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "theme/types.ts")]
pub struct ColorThemeInfo {
    #[ts(type = "ThemeId")]
    pub identifier: ThemeId,
    pub display_name: String,
    #[ts(type = "ThemeMode")]
    pub mode: ThemeMode,
    pub order: Option<isize>, // DEPRECATED
    pub source: PathBuf,
    pub is_default: Option<bool>, // DEPRECATED
}
