pub mod workspace;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
#[ts(export, export_to = "main/types.ts")]
pub enum OpenInTarget {
    #[default]
    #[serde(rename = "NEW_WINDOW")]
    NewWindow,

    #[serde(rename = "CURRENT_WINDOW")]
    CurrentWindow,
}
