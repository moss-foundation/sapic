use std::path::PathBuf;

use crate::models::primitives::CollectionId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamCollectionsEvent {
    #[ts(type = "string")]
    pub id: CollectionId,
    pub name: String,
    pub repository: Option<String>,
    pub order: Option<usize>,
    pub picture_path: Option<PathBuf>,
}

/// @category Event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEnvironmentsEvent {
    pub id: String,

    /// The id of the collection that the environment belongs to.
    /// If the environment is global, this will be `None`.
    pub collection_id: Option<String>,

    pub name: String,
    pub order: Option<usize>,
}
