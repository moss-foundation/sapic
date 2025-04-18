use moss_common::leased_slotmap::ResourceKey;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct QueryParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct QueryParamItem {
    pub key: String,
    pub value: String,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: QueryParamOptions,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PathParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PathParamItem {
    pub key: String,
    pub value: String,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: PathParamOptions,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HeaderParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HeaderParamItem {
    pub key: String,
    pub value: String,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: HeaderParamOptions,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestBody {
    Raw(RawBodyType),
    FormData(Vec<FormDataItem>),
    UrlEncoded(Vec<UrlEncodedItem>),
    Binary(String),
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct FormDataItem {
    pub key: String,
    pub value: FormDataValue,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: FormDataOptions,
}

#[derive(Clone, Debug, Serialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum FormDataValue {
    Text(String),
    File(PathBuf),
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct FormDataOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UrlEncodedItem {
    pub key: String,
    pub value: String,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: UrlEncodedOptions,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UrlEncodedOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RawBodyType {
    Text(String),
    Json(String),
    Html(String),
    Xml(String),
}

#[derive(Debug, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestNodeInfo {
    Request {
        key: ResourceKey,
        name: String,
        // This field will be encoded if the input relative path contains special characters
        path: PathBuf,
        order: Option<usize>,
        protocol: RequestProtocol,
    },
    Group {
        key: ResourceKey,
        name: String,
        // This field will be encoded if the input relative path contains special characters
        path: PathBuf,
        order: Option<usize>,
    },
}

impl RequestNodeInfo {
    pub fn name(&self) -> &str {
        match self {
            RequestNodeInfo::Request { name, .. } => name,
            RequestNodeInfo::Group { name, .. } => name,
        }
    }

    pub fn path(&self) -> &PathBuf {
        match self {
            RequestNodeInfo::Request { path, .. } => path,
            RequestNodeInfo::Group { path, .. } => path,
        }
    }

    pub fn order(&self) -> &Option<usize> {
        match self {
            RequestNodeInfo::Request { order, .. } => order,
            RequestNodeInfo::Group { order, .. } => order,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum HttpMethod {
    Post,
    Get,
    Put,
    Delete,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestProtocol {
    Http(HttpMethod),
    WebSocket,
    GraphQL,
    Grpc,
}

impl RequestProtocol {
    pub fn is_http(&self) -> bool {
        match self {
            RequestProtocol::Http(_) => true,
            _ => false,
        }
    }
}
