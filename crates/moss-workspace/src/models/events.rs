use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamCollectionsEvent {
    pub id: Uuid,
    pub name: String,
    pub repository: Option<String>,
    pub order: Option<usize>,
    pub picture_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEnvironmentsEvent {
    pub id: Uuid,

    /// The id of the collection that the environment belongs to.
    /// If the environment is global, this will be `None`.
    pub collection_id: Option<Uuid>,

    pub name: String,
    pub order: Option<usize>,
}
