use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

use crate::primitives::ThemeId;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ThemeDescriptor {
    pub identifier: ThemeId,
    pub display_name: String,
    pub order: usize,
    pub source: PathBuf,
}
