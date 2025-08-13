use derive_more::Deref;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf, sync::Arc};
use ts_rs::TS;

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntryId(Arc<String>);
impl EntryId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for EntryId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for EntryId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for EntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[rustfmt::skip]
impl TS for EntryId {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name() -> String { "string".to_string() }
    fn inline() -> String { "string".to_string() }
    fn inline_flattened() -> String { "string".to_string() }
    fn decl() -> String { unreachable!() }
    fn decl_concrete() -> String { unreachable!() }
    fn dependencies() -> Vec<ts_rs::Dependency> { vec![] }
}

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QueryParamId(Arc<String>);
impl QueryParamId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for QueryParamId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for QueryParamId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for QueryParamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[rustfmt::skip]
impl TS for QueryParamId {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name() -> String { "string".to_string() }
    fn inline() -> String { "string".to_string() }
    fn inline_flattened() -> String { "string".to_string() }
    fn decl() -> String { unreachable!() }
    fn decl_concrete() -> String { unreachable!() }
    fn dependencies() -> Vec<ts_rs::Dependency> { vec![] }
}

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PathParamId(Arc<String>);
impl PathParamId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for PathParamId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for PathParamId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for PathParamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[rustfmt::skip]
impl TS for PathParamId {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name() -> String { "string".to_string() }
    fn inline() -> String { "string".to_string() }
    fn inline_flattened() -> String { "string".to_string() }
    fn decl() -> String { unreachable!() }
    fn decl_concrete() -> String { unreachable!() }
    fn dependencies() -> Vec<ts_rs::Dependency> { vec![] }
}

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HeaderId(Arc<String>);
impl HeaderId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for HeaderId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for HeaderId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for HeaderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[rustfmt::skip]
impl TS for HeaderId {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name() -> String { "string".to_string() }
    fn inline() -> String { "string".to_string() }
    fn inline_flattened() -> String { "string".to_string() }
    fn decl() -> String { unreachable!() }
    fn decl_concrete() -> String { unreachable!() }
    fn dependencies() -> Vec<ts_rs::Dependency> { vec![] }
}

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename = "EntryPath", rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub struct FrontendEntryPath {
    pub raw: PathBuf,
    pub segments: Vec<String>,
}

impl FrontendEntryPath {
    pub fn new(raw: PathBuf) -> Self {
        let segments = raw
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();

        Self { raw, segments }
    }
}

/// @category Primitive
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum EntryClass {
    Request,
    Endpoint,
    Component,
    Schema,
}

impl ToString for EntryClass {
    fn to_string(&self) -> String {
        match self {
            EntryClass::Request => "request".to_string(),
            EntryClass::Endpoint => "endpoint".to_string(),
            EntryClass::Component => "component".to_string(),
            EntryClass::Schema => "schema".to_string(),
        }
    }
}

/// @category Primitive
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum EntryKind {
    Dir,
    Item,
    Case,
}

/// @category Primitive
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

impl ToString for EntryProtocol {
    fn to_string(&self) -> String {
        match self {
            EntryProtocol::Get => "Get".to_string(),
            EntryProtocol::Post => "Post".to_string(),
            EntryProtocol::Put => "Put".to_string(),
            EntryProtocol::Delete => "Delete".to_string(),
            EntryProtocol::WebSocket => "WebSocket".to_string(),
            EntryProtocol::Graphql => "Graphql".to_string(),
            EntryProtocol::Grpc => "Grpc".to_string(),
        }
    }
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

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}
