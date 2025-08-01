use std::path::PathBuf;

use crate::models::primitives::CollectionId;
use moss_environment::models::primitives::EnvironmentId;
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
    pub order: Option<isize>,
    pub expanded: bool,
    pub repository: Option<String>,
    pub picture_path: Option<PathBuf>,
}

/// @category Event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEnvironmentsEvent {
    #[ts(type = "string")]
    pub id: EnvironmentId,

    /// The id of the collection that the environment belongs to.
    /// If the environment is global, this will be `None`.
    #[ts(type = "string")]
    pub collection_id: Option<CollectionId>,

    pub name: String,
    pub order: isize,
}
