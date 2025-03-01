use serde::Serialize;
use serde_json::Value as JsonValue;
use ts_rs::TS;
use crate::models::collection::HttpRequestType::{Delete, Get, Post, Put};
use crate::models::collection::RequestType;

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types/request.ts")]
pub enum HttpMethod {
    Post,
    Get,
    Put,
    Delete,
}

impl Into<RequestType> for HttpMethod {
    fn into(self) -> RequestType {
        match self {
            HttpMethod::Post => {RequestType::Http(Post)}
            HttpMethod::Get => {RequestType::Http(Get)}
            HttpMethod::Put => {RequestType::Http(Put)}
            HttpMethod::Delete => {RequestType::Http(Delete)}
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types/request.ts")]
pub struct QueryParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types/request.ts")]
pub struct QueryParamItem {
    pub key: String,
    pub value: JsonValue,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: QueryParamOptions,
}
