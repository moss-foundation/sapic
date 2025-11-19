use moss_id_macro::ids;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

ids!([
    ProjectId,
    ResourceId,
    QueryParamId,
    PathParamId,
    HeaderId,
    FormDataParamId,
    UrlencodedParamId,
]);

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename = "ResourcePath", rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub struct FrontendResourcePath {
    pub raw: PathBuf,
    pub segments: Vec<String>,
}

impl FrontendResourcePath {
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
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub enum ResourceClass {
    Endpoint,
    Component,
    Schema,
}

impl ToString for ResourceClass {
    fn to_string(&self) -> String {
        match self {
            ResourceClass::Endpoint => "endpoint".to_string(),
            ResourceClass::Component => "component".to_string(),
            ResourceClass::Schema => "schema".to_string(),
        }
    }
}

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum ResourceKind {
    Dir,
    Item,
    Case,
}

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum ResourceProtocol {
    Get,
    Post,
    Put,
    Delete,
    WebSocket,
    Graphql,
    Grpc,
}

impl ToString for ResourceProtocol {
    fn to_string(&self) -> String {
        match self {
            ResourceProtocol::Get => "Get".to_string(),
            ResourceProtocol::Post => "Post".to_string(),
            ResourceProtocol::Put => "Put".to_string(),
            ResourceProtocol::Delete => "Delete".to_string(),
            ResourceProtocol::WebSocket => "WebSocket".to_string(),
            ResourceProtocol::Graphql => "Graphql".to_string(),
            ResourceProtocol::Grpc => "Grpc".to_string(),
        }
    }
}

impl From<&HttpMethod> for ResourceProtocol {
    fn from(method: &HttpMethod) -> Self {
        match method {
            HttpMethod::Get => ResourceProtocol::Get,
            HttpMethod::Post => ResourceProtocol::Post,
            HttpMethod::Put => ResourceProtocol::Put,
            HttpMethod::Delete => ResourceProtocol::Delete,
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
