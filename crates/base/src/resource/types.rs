use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

use crate::resource::types::primitives::*;

pub mod primitives;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "resource/types.ts")]
pub struct ResourceSummary {
    pub id: ResourceId,
    pub name: String,
    pub path: PathBuf,
    pub class: ResourceClass,
    pub kind: ResourceKind,
    pub protocol: Option<ResourceProtocol>,
}
