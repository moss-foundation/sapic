pub mod workspace;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
#[ts(export, export_to = "main/types.ts")]
#[serde(rename_all = "UPPERCASE")]
pub enum OpenInTarget {
    #[default]
    NewWindow,

    CurrentWindow,
}
