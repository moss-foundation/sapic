use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
}
