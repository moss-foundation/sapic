pub mod primitives;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::language::types::primitives::{LanguageCode, LanguageDirection};

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "language/types.ts")]
pub struct LanguageInfo {
    pub display_name: String,
    pub code: LanguageCode,
    pub direction: Option<LanguageDirection>,
}
