use moss_common::models::primitives::Identifier;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use uuid::Uuid;

use super::primitives::EntryId;

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
