use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ChangeUsize {
    Update(usize),
    Remove,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ChangeString {
    Update(String),
    Remove,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ChangeBool {
    Update(bool),
    Remove,
}
