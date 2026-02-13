use moss_id_macro::ids;
use sapic_base::resource::types::primitives::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ids!([
    QueryParamId,
    PathParamId,
    HeaderId,
    FormDataParamId,
    UrlencodedParamId,
]);

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
