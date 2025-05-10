use moss_common::identifier::Identifier;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct WorkspaceInfo {
    pub id: Identifier,
    pub display_name: String,
    #[ts(optional)]
    pub last_opened_at: Option<i64>,
}
