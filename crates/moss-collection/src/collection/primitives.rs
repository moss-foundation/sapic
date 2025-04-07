use anyhow::anyhow;
use std::ops::Deref;

use crate::models::types::{HttpMethod, RequestProtocol};

const HTTP_POST_EXT: &str = "post";
const HTTP_GET_EXT: &str = "get";
const HTTP_PUT_EXT: &str = "put";
const HTTP_DELETE_EXT: &str = "del";

const WEBSOCKET_EXT: &str = "ws";
const GRAPHQL_EXT: &str = "gql";
const GRPC_EXT: &str = "grpc";

pub struct FileExt(&'static str);

impl Default for FileExt {
    fn default() -> Self {
        Self(HTTP_GET_EXT)
    }
}

impl Deref for FileExt {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl ToString for FileExt {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<&str> for FileExt {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            HTTP_POST_EXT => Ok(FileExt(HTTP_POST_EXT)),
            HTTP_GET_EXT => Ok(FileExt(HTTP_GET_EXT)),
            HTTP_PUT_EXT => Ok(FileExt(HTTP_PUT_EXT)),
            HTTP_DELETE_EXT => Ok(FileExt(HTTP_DELETE_EXT)),
            WEBSOCKET_EXT => Ok(FileExt(WEBSOCKET_EXT)),
            GRAPHQL_EXT => Ok(FileExt(GRAPHQL_EXT)),
            GRPC_EXT => Ok(FileExt(GRPC_EXT)),
            _ => Err(anyhow!("Invalid endpoint file extension: {}", value)),
        }
    }
}

impl From<&RequestProtocol> for FileExt {
    fn from(protocol: &RequestProtocol) -> Self {
        match protocol {
            RequestProtocol::Http(HttpMethod::Post) => FileExt(HTTP_POST_EXT),
            RequestProtocol::Http(HttpMethod::Get) => FileExt(HTTP_GET_EXT),
            RequestProtocol::Http(HttpMethod::Put) => FileExt(HTTP_PUT_EXT),
            RequestProtocol::Http(HttpMethod::Delete) => FileExt(HTTP_DELETE_EXT),
            RequestProtocol::WebSocket => FileExt(WEBSOCKET_EXT),
            RequestProtocol::GraphQL => FileExt(GRAPHQL_EXT),
            RequestProtocol::Grpc => FileExt(GRPC_EXT),
        }
    }
}

impl From<FileExt> for RequestProtocol {
    fn from(ext: FileExt) -> Self {
        match ext.0 {
            HTTP_POST_EXT => RequestProtocol::Http(HttpMethod::Post),
            HTTP_GET_EXT => RequestProtocol::Http(HttpMethod::Get),
            HTTP_PUT_EXT => RequestProtocol::Http(HttpMethod::Put),
            HTTP_DELETE_EXT => RequestProtocol::Http(HttpMethod::Delete),
            WEBSOCKET_EXT => RequestProtocol::WebSocket,
            GRAPHQL_EXT => RequestProtocol::GraphQL,
            GRPC_EXT => RequestProtocol::Grpc,
            _ => unreachable!(),
        }
    }
}
