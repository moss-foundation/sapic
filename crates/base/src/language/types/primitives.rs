use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type LanguageCode = String;

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "language/primitives.ts")]
pub enum LanguageDirection {
    #[serde(rename = "ltr")]
    LtR,
    #[serde(rename = "rtl")]
    RtL,
}
