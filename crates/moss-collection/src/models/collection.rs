use anyhow::anyhow;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum HttpRequestType {
    Post,
    Get,
    Put,
    Delete,
}

impl ToString for HttpRequestType {
    fn to_string(&self) -> String {
        match self {
            HttpRequestType::Post => "post".to_string(),
            HttpRequestType::Get => "get".to_string(),
            HttpRequestType::Put => "put".to_string(),
            HttpRequestType::Delete => "del".to_string(),
        }
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

impl ToString for RequestType {
    fn to_string(&self) -> String {
        match self {
            RequestType::Http(http_request_type) => http_request_type.to_string(),
            RequestType::WebSocket => "ws".to_string(),
            RequestType::GraphQL => "gql".to_string(),
            RequestType::Grpc => "grpc".to_string(),
            RequestType::Variant => "variant".to_string(),
        }
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
