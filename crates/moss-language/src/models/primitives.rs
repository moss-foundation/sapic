use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

/// @category Primitive
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, TS, Display)]
#[ts(export, export_to = "primitives.ts")]
pub struct LanguageId(Arc<String>);

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum Direction {
    #[serde(rename = "ltr")]
    LtR,
    #[serde(rename = "rtl")]
    RtL,
}
