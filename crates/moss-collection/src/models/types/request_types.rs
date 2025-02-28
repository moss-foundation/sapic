use serde::Serialize;
use serde_json::Value as JsonValue;
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types/request.ts")]
pub enum HttpMethod {
    Post,
    Get,
    Put,
    Delete,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types/request.ts")]
pub struct QueryParamOptions {
    pub propagate: bool,
}

#[derive(Debug, Serialize, TS)]
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
