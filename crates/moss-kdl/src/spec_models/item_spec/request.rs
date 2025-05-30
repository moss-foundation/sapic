use crate::foundations::http::HttpRequestFile;
use kdl::KdlDocument;

#[derive(Clone)]
pub enum RequestContent {
    Http(HttpRequestFile),
}

impl Into<KdlDocument> for RequestContent {
    fn into(self) -> KdlDocument {
        match self {
            RequestContent::Http(http) => http.into(),
        }
    }
}
