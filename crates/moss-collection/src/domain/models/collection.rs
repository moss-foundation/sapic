use anyhow::anyhow;

#[derive(Debug, Clone)]
pub enum HttpRequestType {
    Post,
    Get,
    Put,
    Delete,
}

#[derive(Debug, Clone)]
pub enum RequestType {
    Http(HttpRequestType),
    WebSocket,
    Graphql,
    Grpc,
    Variant,
}

impl TryFrom<&str> for RequestType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "post" => Ok(Self::Http(HttpRequestType::Post)),
            "get" => Ok(Self::Http(HttpRequestType::Get)),
            "put" => Ok(Self::Http(HttpRequestType::Put)),
            "delete" => Ok(Self::Http(HttpRequestType::Delete)),

            "ws" => Ok(Self::WebSocket),
            "graphql" => Ok(Self::WebSocket),
            "grpc" => Ok(Self::WebSocket),

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
