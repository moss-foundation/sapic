use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub enum Scope {
    App,
    Workspace(String),
}
