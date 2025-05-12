use serde::{Deserialize, Serialize};
use ts_rs::TS;

use moss_common::models::primitives::Identifier;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct StreamCollectionsEvent {
    pub id: Identifier,
    pub display_name: String,
    pub order: Option<usize>,
}
