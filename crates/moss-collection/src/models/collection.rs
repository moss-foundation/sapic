use std::fmt::Display;
use anyhow::anyhow;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum HttpRequestType {
    Post,
    Get,
    Put,
    Delete,
}

impl Display for HttpRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", match self {
            HttpRequestType::Post => "post",
            HttpRequestType::Get => "get",
            HttpRequestType::Put => "put",
            HttpRequestType::Delete => "del",
        })
    }
}

#[derive(Debug, Clone)]
pub enum RequestType {
    Http(HttpRequestType),
    WebSocket,
    GraphQL,
    Grpc,
    Variant,
}

impl Default for RequestType {
    fn default() -> Self {
        Self::Http(HttpRequestType::Get)
    }
}

impl Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", match self {
            Self::Http(http_request_type) => http_request_type.to_string(),
            Self::WebSocket => "ws".to_string(),
            Self::GraphQL => "gql".to_string(),
            Self::Grpc => "grpc".to_string(),
            Self::Variant => "variant".to_string(),
        })
    }
}

impl TryFrom<&str> for RequestType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "post" => Ok(Self::Http(HttpRequestType::Post)),
            "get" => Ok(Self::Http(HttpRequestType::Get)),
            "put" => Ok(Self::Http(HttpRequestType::Put)),
            "del" => Ok(Self::Http(HttpRequestType::Delete)),

            "ws" => Ok(Self::WebSocket),
            "gql" => Ok(Self::GraphQL),
            "grpc" => Ok(Self::Grpc),

            "variant" => Ok(Self::Variant),

            _ => Err(anyhow!("unknown request file type extension: {}", value)),
        }
    }
}

impl RequestType {
    pub fn is_http(&self) -> bool {
        match self {
            RequestType::Http(_) => true,
            _ => false,
        }
    }

    pub fn is_variant(&self) -> bool {
        match self {
            RequestType::Variant => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CollectionRequestVariantEntry {
    pub name: String,
    pub order: Option<usize>,
}
