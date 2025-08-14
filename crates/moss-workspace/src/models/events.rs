use std::path::PathBuf;

use moss_environment::models::primitives::EnvironmentId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{primitives::CollectionId, types::BranchInfo};

/// @category Event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamCollectionsEvent {
    pub id: CollectionId,
    pub name: String,
    pub order: Option<isize>,
    pub expanded: bool,
    pub repository: Option<String>,

    pub branch: Option<BranchInfo>,

    pub picture_path: Option<PathBuf>,
}

/// @category Event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEnvironmentsEvent {
    pub id: EnvironmentId,

    /// The id of the collection that the environment belongs to.
    /// If the environment is global, this will be `None`.
    pub collection_id: Option<CollectionId>,

    pub name: String,
    pub order: Option<isize>,
    pub expanded: bool,
}
