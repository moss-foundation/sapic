pub mod primitives;
pub use primitives::*;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "resource/types.ts")]
pub struct ResourceSummary {
    pub id: ResourceId,
    pub name: String,
    pub path: PathBuf,
    pub class: ResourceClass,
    pub kind: ResourceKind,
    pub protocol: Option<ResourceProtocol>,
}
