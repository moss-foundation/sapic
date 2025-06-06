use moss_common::models::primitives::Identifier;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use uuid::Uuid;

use super::primitives::EntryId;

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum HttpMethod {
    Post,
    Get,
    Put,
    Delete,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Post => "post",
            HttpMethod::Get => "get",
            HttpMethod::Put => "put",
            HttpMethod::Delete => "del",
        }
    }
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

impl Default for RequestProtocol {
    fn default() -> Self {
        Self::Http(HttpMethod::Get)
    }
}

impl RequestProtocol {
    pub fn is_http(&self) -> bool {
        match self {
            RequestProtocol::Http(_) => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RequestProtocol::Http(method) => method.as_str(),
            RequestProtocol::WebSocket => "websocket",
            RequestProtocol::GraphQL => "graphql",
            RequestProtocol::Grpc => "grpc",
        }
    }

    pub fn to_filename(&self) -> String {
        format!("{}.sapic", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, PartialEq)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum PathChangeKind {
    Loaded,
    Created,
    Removed,
    Updated,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EntryKind {
    Dir,
    File,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum Classification {
    Request,
    Endpoint,
    Component,
    Schema,
}

impl Classification {
    pub fn as_str(&self) -> &'static str {
        match self {
            Classification::Request => "request",
            Classification::Endpoint => "endpoint",
            Classification::Component => "component",
            Classification::Schema => "schema",
        }
    }
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EntryInfo {
    pub id: EntryId,
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub classification: Classification,
    #[ts(optional)]
    pub protocol: Option<RequestProtocol>,
    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfo {
    pub id: Uuid,
    pub name: String,
    #[ts(optional)]
    pub order: Option<usize>,
}
