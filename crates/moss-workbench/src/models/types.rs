use moss_common::models::primitives::Identifier;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct WorkspaceInfo {
    #[ts(type = "Identifier")]
    pub id: Identifier,
    pub display_name: String,
    #[ts(optional)]
    pub last_opened_at: Option<i64>,
}
