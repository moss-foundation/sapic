use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ConfigurationTarget {
    Profile,
    Workspace,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "primitives.ts")]
pub enum ConfigurationParameterType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}
