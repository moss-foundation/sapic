pub mod primitives;

use serde::Serialize;
use std::{path::Path, sync::Arc};
use ts_rs::TS;

use crate::workspace::types::primitives::WorkspaceId;

pub struct WorkspaceSummary {
    pub name: String,
}

/// @category Type
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct WorkspaceInfo {
    pub id: WorkspaceId,
    pub name: String,
    pub last_opened_at: Option<i64>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}
