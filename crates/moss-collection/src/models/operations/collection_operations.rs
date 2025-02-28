use serde::Serialize;
use std::path::PathBuf;
use ts_rs::TS;

use crate::models::types::request_types::QueryParamItem;

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct CreateCollectionInput {
    pub name: String,
    pub path: PathBuf,
    #[ts(optional)]
    pub repo: Option<String>, // Url ?
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct OverviewCollectionOutput {
    pub name: String,
    pub path: PathBuf,
    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct CreateRequestInput {
    pub name: String,
    #[ts(optional)]
    pub url: Option<String>, // Url ?
    pub query_params: Vec<QueryParamItem>,
}
