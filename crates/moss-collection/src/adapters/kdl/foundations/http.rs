use kdl::KdlValue;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub enum HttpMethod {
    Post,
    Put,
    #[default]
    Get,
    Delete,
}

#[derive(Debug)]
pub struct Metadata {
    pub order: Option<usize>,
    pub method: HttpMethod,
}

#[derive(Debug)]
pub struct Url {
    pub raw: Option<String>,
    pub host: Option<String>,
}

#[derive(Debug, Default)]
pub struct QueryParamBody {
    pub value: Option<KdlValue>,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: QueryParamOptions,
}

#[derive(Debug, Default)]
pub struct QueryParamOptions {
    pub propagate: bool,
}

#[derive(Debug, Default)]
pub struct PathParamBody {
    pub value: Option<KdlValue>,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: PathParamOptions,
}

#[derive(Debug, Default)]
pub struct PathParamOptions {
    pub propagate: bool,
}

#[derive(Debug, Default)]
pub struct HeaderBody {
    pub value: Option<KdlValue>,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub disabled: bool,
    pub options: HeaderOptions,
}

#[derive(Debug, Default)]
pub struct HeaderOptions {
    pub propagate: bool,
}

#[derive(Debug, Default)]
pub struct Request {
    pub metadata: Option<Metadata>,
    pub url: Option<Url>,
    pub query_params: Option<HashMap<String, QueryParamBody>>,
    pub path_params: Option<HashMap<String, PathParamBody>>,
    pub headers: Option<HashMap<String, HeaderBody>>,
}
