use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub struct EntryPath {
    pub raw: PathBuf,
    pub segments: Vec<String>,
}

impl EntryPath {
    pub fn new(raw: PathBuf) -> Self {
        let segments = raw
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();

        Self { raw, segments }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
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

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum EntryKind {
    Dir,
    Item,
    Case,
}

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
