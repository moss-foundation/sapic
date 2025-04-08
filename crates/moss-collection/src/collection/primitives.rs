// use anyhow::anyhow;
// use std::ops::Deref;

// use crate::models::types::{HttpMethod, RequestProtocol};

// const ENTRY_HTTP_SPEC_FILENAME: &str = "post";
// const ENTRY_GET_SPEC_FILENAME: &str = "get";
// const ENTRY_PUT_SPEC_FILENAME: &str = "put";
// const ENTRY_DELETE_SPEC_FILENAME: &str = "del";

// const ENTRY_WEBSOCKET_SPEC_FILENAME: &str = "ws";
// const ENTRY_GRAPHQL_SPEC_FILENAME: &str = "gql";
// const ENTRY_GRPC_SPEC_FILENAME: &str = "grpc";

// const ENTRY_FOLDER_SPEC_FILENAME: &str = "folder";

// pub struct CollectionEntryFilename(&'static str);

// impl Default for CollectionEntryFilename {
//     fn default() -> Self {
//         Self(ENTRY_GET_SPEC_FILENAME)
//     }
// }

// impl Deref for CollectionEntryFilename {
//     type Target = str;

//     fn deref(&self) -> &Self::Target {
//         self.0
//     }
// }

// impl ToString for CollectionEntryFilename {
//     fn to_string(&self) -> String {
//         self.0.to_string()
//     }
// }

// impl TryFrom<&str> for CollectionEntryFilename {
//     type Error = anyhow::Error;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             ENTRY_HTTP_SPEC_FILENAME => Ok(CollectionEntryFilename(ENTRY_HTTP_SPEC_FILENAME)),
//             ENTRY_GET_SPEC_FILENAME => Ok(CollectionEntryFilename(ENTRY_GET_SPEC_FILENAME)),
//             ENTRY_PUT_SPEC_FILENAME => Ok(CollectionEntryFilename(ENTRY_PUT_SPEC_FILENAME)),
//             ENTRY_DELETE_SPEC_FILENAME => Ok(CollectionEntryFilename(ENTRY_DELETE_SPEC_FILENAME)),
//             ENTRY_WEBSOCKET_SPEC_FILENAME => {
//                 Ok(CollectionEntryFilename(ENTRY_WEBSOCKET_SPEC_FILENAME))
//             }
//             ENTRY_GRAPHQL_SPEC_FILENAME => Ok(CollectionEntryFilename(ENTRY_GRAPHQL_SPEC_FILENAME)),
//             ENTRY_GRPC_SPEC_FILENAME => Ok(CollectionEntryFilename(ENTRY_GRPC_SPEC_FILENAME)),
//             ENTRY_FOLDER_SPEC_FILENAME => Ok(CollectionEntryFilename(ENTRY_FOLDER_SPEC_FILENAME)),
//             _ => Err(anyhow!("Invalid endpoint file extension: {}", value)),
//         }
//     }
// }

// impl From<&RequestProtocol> for CollectionEntryFilename {
//     fn from(protocol: &RequestProtocol) -> Self {
//         match protocol {
//             RequestProtocol::Http(HttpMethod::Post) => {
//                 CollectionEntryFilename(ENTRY_HTTP_SPEC_FILENAME)
//             }
//             RequestProtocol::Http(HttpMethod::Get) => {
//                 CollectionEntryFilename(ENTRY_GET_SPEC_FILENAME)
//             }
//             RequestProtocol::Http(HttpMethod::Put) => {
//                 CollectionEntryFilename(ENTRY_PUT_SPEC_FILENAME)
//             }
//             RequestProtocol::Http(HttpMethod::Delete) => {
//                 CollectionEntryFilename(ENTRY_DELETE_SPEC_FILENAME)
//             }
//             RequestProtocol::WebSocket => CollectionEntryFilename(ENTRY_WEBSOCKET_SPEC_FILENAME),
//             RequestProtocol::GraphQL => CollectionEntryFilename(ENTRY_GRAPHQL_SPEC_FILENAME),
//             RequestProtocol::Grpc => CollectionEntryFilename(ENTRY_GRPC_SPEC_FILENAME),
//         }
//     }
// }

// impl From<CollectionEntryFilename> for RequestProtocol {
//     fn from(ext: CollectionEntryFilename) -> Self {
//         match ext.0 {
//             ENTRY_HTTP_SPEC_FILENAME => RequestProtocol::Http(HttpMethod::Post),
//             ENTRY_GET_SPEC_FILENAME => RequestProtocol::Http(HttpMethod::Get),
//             ENTRY_PUT_SPEC_FILENAME => RequestProtocol::Http(HttpMethod::Put),
//             ENTRY_DELETE_SPEC_FILENAME => RequestProtocol::Http(HttpMethod::Delete),
//             ENTRY_WEBSOCKET_SPEC_FILENAME => RequestProtocol::WebSocket,
//             ENTRY_GRAPHQL_SPEC_FILENAME => RequestProtocol::GraphQL,
//             ENTRY_GRPC_SPEC_FILENAME => RequestProtocol::Grpc,
//             _ => unreachable!(),
//         }
//     }
// }
