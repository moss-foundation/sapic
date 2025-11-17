use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// DEPRECATED
/// Should be removed when its unused
///
/// @category Primitive
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "configuration/primitives.ts")]
pub enum ConfigurationTarget {
    User,
    Workspace,
}

/// @category Primitive
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "configuration/primitives.ts")]
pub enum ConfigurationParameterType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}
