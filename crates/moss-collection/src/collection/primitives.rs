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

pub struct EndpointFileExt(&'static str);

impl Default for EndpointFileExt {
    fn default() -> Self {
        Self(HTTP_GET_EXT)
    }
}

impl Deref for EndpointFileExt {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl ToString for EndpointFileExt {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<&str> for EndpointFileExt {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            HTTP_POST_EXT => Ok(EndpointFileExt(HTTP_POST_EXT)),
            HTTP_GET_EXT => Ok(EndpointFileExt(HTTP_GET_EXT)),
            HTTP_PUT_EXT => Ok(EndpointFileExt(HTTP_PUT_EXT)),
            HTTP_DELETE_EXT => Ok(EndpointFileExt(HTTP_DELETE_EXT)),
            WEBSOCKET_EXT => Ok(EndpointFileExt(WEBSOCKET_EXT)),
            GRAPHQL_EXT => Ok(EndpointFileExt(GRAPHQL_EXT)),
            GRPC_EXT => Ok(EndpointFileExt(GRPC_EXT)),
            _ => Err(anyhow!("Invalid endpoint file extension: {}", value)),
        }
    }
}

impl From<&RequestProtocol> for EndpointFileExt {
    fn from(protocol: &RequestProtocol) -> Self {
        match protocol {
            RequestProtocol::Http(HttpMethod::Post) => EndpointFileExt(HTTP_POST_EXT),
            RequestProtocol::Http(HttpMethod::Get) => EndpointFileExt(HTTP_GET_EXT),
            RequestProtocol::Http(HttpMethod::Put) => EndpointFileExt(HTTP_PUT_EXT),
            RequestProtocol::Http(HttpMethod::Delete) => EndpointFileExt(HTTP_DELETE_EXT),
            RequestProtocol::WebSocket => EndpointFileExt(WEBSOCKET_EXT),
            RequestProtocol::GraphQL => EndpointFileExt(GRAPHQL_EXT),
            RequestProtocol::Grpc => EndpointFileExt(GRPC_EXT),
        }
    }
}

impl From<EndpointFileExt> for RequestProtocol {
    fn from(ext: EndpointFileExt) -> Self {
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
