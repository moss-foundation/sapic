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
pub struct Request {
    pub metadata: Option<Metadata>,
    pub url: Option<Url>,
}
