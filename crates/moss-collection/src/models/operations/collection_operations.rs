use crate::models::types::request_types::{
    HeaderParamItem, HttpMethod, PathParamItem, QueryParamItem, RequestBody,
};
use serde::Serialize;
use std::path::PathBuf;
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct CreateCollectionInput {
    pub name: String,
    pub path: PathBuf,
    #[ts(optional)]
    pub repo: Option<String>,
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

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub enum CreateRequestProtocolSpecificPayload {
    Http {
        method: HttpMethod,
        query_params: Vec<QueryParamItem>,
        path_params: Vec<PathParamItem>,
        headers: Vec<HeaderParamItem>,
        body: Option<RequestBody>,
    },
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct CreateRequestInput {
    pub name: String,
    #[ts(optional)]
    pub relative_path: Option<PathBuf>,
    #[ts(optional)]
    pub url: Option<String>,
    #[ts(optional)]
    pub payload: Option<CreateRequestProtocolSpecificPayload>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct CreateRequestOutput {
    pub key: u64,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct RenameRequestInput {
    pub key: u64,
    pub new_name: String,
}
