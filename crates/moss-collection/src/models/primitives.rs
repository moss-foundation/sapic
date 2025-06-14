use serde::{Deserialize, Serialize};
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
