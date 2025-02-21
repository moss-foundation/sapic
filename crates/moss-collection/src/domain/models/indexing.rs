use anyhow::anyhow;
use std::path::PathBuf;

#[derive(Debug)]
pub enum HttpFileTypeExt {
    Post,
    Get,
    Put,
    Delete,
}

#[derive(Debug)]
pub enum RequestFileTypeExt {
    Http(HttpFileTypeExt),
    WebSocket,
    Graphql,
    Grpc,
    Variant,
}

impl TryFrom<&str> for RequestFileTypeExt {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "post" => Ok(Self::Http(HttpFileTypeExt::Post)),
            "get" => Ok(Self::Http(HttpFileTypeExt::Get)),
            "put" => Ok(Self::Http(HttpFileTypeExt::Put)),
            "delete" => Ok(Self::Http(HttpFileTypeExt::Delete)),

            "ws" => Ok(Self::WebSocket),
            "graphql" => Ok(Self::WebSocket),
            "grpc" => Ok(Self::WebSocket),

            "variant" => Ok(Self::Variant),

            _ => Err(anyhow!("unknown request file type extension: {}", value)),
        }
    }
}

impl RequestFileTypeExt {
    pub fn is_http(&self) -> bool {
        match self {
            RequestFileTypeExt::Http(_) => true,
            _ => false,
        }
    }

    pub fn is_variant(&self) -> bool {
        match self {
            RequestFileTypeExt::Variant => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct RequestVariantEntry {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct RequestEntry {
    pub name: String,
    pub ext: Option<RequestFileTypeExt>,
    pub path: Option<PathBuf>,
    pub variants: Vec<RequestVariantEntry>,
}

#[derive(Debug)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub children: Vec<RequestIndexEntry>,
}

#[derive(Debug)]
pub enum RequestIndexEntry {
    Request(RequestEntry),
    Dir(DirEntry),
}

#[derive(Debug)]
pub struct IndexedCollection {
    pub name: Option<String>,
    pub requests: Vec<RequestIndexEntry>,
}
