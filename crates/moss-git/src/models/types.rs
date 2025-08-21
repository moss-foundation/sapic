use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct BranchInfo {
    pub name: String,
    pub ahead: Option<usize>,
    pub behind: Option<usize>,
}
