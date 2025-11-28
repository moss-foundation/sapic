use moss_id_macro::ids;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ids!([WorkspaceId]);

/// @category Type
#[derive(Debug, PartialEq, Serialize, Deserialize, TS, Clone)]
#[ts(export, export_to = "primitives.ts")]
pub enum WorkspaceMode {
    #[serde(rename = "LIVE")]
    Live,

    #[serde(rename = "DESIGN")]
    Design,
}

impl Default for WorkspaceMode {
    fn default() -> Self {
        Self::Live
    }
}
