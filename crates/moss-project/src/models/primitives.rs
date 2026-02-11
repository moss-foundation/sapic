use moss_id_macro::ids;
use sapic_base::resource::types::primitives::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

ids!([
    QueryParamId,
    PathParamId,
    HeaderId,
    FormDataParamId,
    UrlencodedParamId,
]);

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename = "ResourcePath", rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub struct FrontendResourcePath {
    pub raw: PathBuf,
    pub segments: Vec<String>,
}

impl FrontendResourcePath {
    pub fn new(raw: PathBuf) -> Self {
        let segments = raw
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();

        Self { raw, segments }
    }
}

impl From<&HttpMethod> for ResourceProtocol {
    fn from(method: &HttpMethod) -> Self {
        match method {
            HttpMethod::Get => ResourceProtocol::Get,
            HttpMethod::Post => ResourceProtocol::Post,
            HttpMethod::Put => ResourceProtocol::Put,
            HttpMethod::Delete => ResourceProtocol::Delete,
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
