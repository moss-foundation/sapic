use moss_common::models::primitives::Identifier;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "events.ts")]
pub struct StreamCollectionsEvent {
    pub id: Uuid,
    pub name: String,
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "events.ts")]
pub struct ListEnvironmentsEvent {
    pub id: Identifier,

    /// The id of the collection that the environment belongs to.
    /// If the environment is global, this will be `None`.
    #[ts(optional)]
    pub collection_id: Option<Uuid>,

    pub name: String,
    #[ts(optional)]
    pub order: Option<usize>,
}
