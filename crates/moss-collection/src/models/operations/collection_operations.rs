use crate::models::types::request_types::{
    HeaderParamItem, HttpMethod, PathParamItem, QueryParamItem, RequestBody,
};
use serde::Serialize;
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;
use crate::models::collection::RequestType;

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

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct CreateRequestInput {
    #[validate(length(min = 1))]
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

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct RenameRequestInput {
    pub key: u64,
    #[validate(length(min = 1))]
    pub new_name: String,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct DeleteRequestInput {
    pub key: u64,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations/collection.ts")]
pub struct ListRequestsOutput(pub Vec<RequestInfo>);

#[derive(Debug, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/collection.ts")]
pub struct RequestInfo {
    // FIXME: What fields should this contain?
    pub key: u64,
    pub name: String,
    pub request_dir_relative_path: PathBuf,
    pub order: Option<usize>,
    pub typ: RequestType
}