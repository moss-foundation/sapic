use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::primitives::{LanguageCode, LanguageDirection};

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct LanguageInfo {
    pub display_name: String,
    pub code: LanguageCode,
    pub direction: Option<LanguageDirection>,
}
