use moss_id_macro::ids;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

ids!([EntryId, QueryParamId, PathParamId, HeaderId]);

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename = "EntryPath", rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub struct FrontendEntryPath {
    pub raw: PathBuf,
    pub segments: Vec<String>,
}

impl FrontendEntryPath {
    pub fn new(raw: PathBuf) -> Self {
        let segments = raw
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();

        Self { raw, segments }
    }
}

/// @category Primitive
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum EntryClass {
    Request,
    Endpoint,
    Component,
    Schema,
}

impl ToString for EntryClass {
    fn to_string(&self) -> String {
        match self {
            EntryClass::Request => "request".to_string(),
            EntryClass::Endpoint => "endpoint".to_string(),
            EntryClass::Component => "component".to_string(),
            EntryClass::Schema => "schema".to_string(),
        }
    }
}

/// @category Primitive
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum EntryKind {
    Dir,
    Item,
    Case,
}

/// @category Primitive
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum EntryProtocol {
    Get,
    Post,
    Put,
    Delete,
    WebSocket,
    Graphql,
    Grpc,
}

impl ToString for EntryProtocol {
    fn to_string(&self) -> String {
        match self {
            EntryProtocol::Get => "Get".to_string(),
            EntryProtocol::Post => "Post".to_string(),
            EntryProtocol::Put => "Put".to_string(),
            EntryProtocol::Delete => "Delete".to_string(),
            EntryProtocol::WebSocket => "WebSocket".to_string(),
            EntryProtocol::Graphql => "Graphql".to_string(),
            EntryProtocol::Grpc => "Grpc".to_string(),
        }
    }
}

impl From<&HttpMethod> for EntryProtocol {
    fn from(method: &HttpMethod) -> Self {
        match method {
            HttpMethod::Get => EntryProtocol::Get,
            HttpMethod::Post => EntryProtocol::Post,
            HttpMethod::Put => EntryProtocol::Put,
            HttpMethod::Delete => EntryProtocol::Delete,
        }
    }
}

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}
