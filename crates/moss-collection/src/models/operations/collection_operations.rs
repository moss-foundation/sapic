use serde::Serialize;
use std::path::PathBuf;
use ts_rs::TS;
use crate::kdl::foundations::http::Url;
use crate::models::types::request_types::{HttpMethod, QueryParamItem};

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct CreateCollectionInput {
    pub name: String,
    pub path: PathBuf,
    #[ts(optional)]
    pub repo: Option<Url>,
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
    },
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct CreateRequestInput {
    pub name: String,
    #[ts(optional)]
    pub url: Option<Url>,
    pub payload: Option<CreateRequestProtocolSpecificPayload>,
}
