use serde::{Deserialize, Serialize};
use strum_macros::Display as ToString;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum EntryClass {
    Request,
    Endpoint,
    Component,
    Schema,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToString, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    #[strum(to_string = "GET")]
    Get,
    #[serde(rename = "POST")]
    #[strum(to_string = "POST")]
    Post,
    #[serde(rename = "PUT")]
    #[strum(to_string = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    #[strum(to_string = "DELETE")]
    Delete,
}
