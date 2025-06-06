use serde::{Deserialize, Serialize};

use crate::configuration::SpecificationMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpDirSpecificationModel {}

pub enum RequestDirSpecificationModel {
    Http(HttpDirSpecificationModel),
}

pub enum DirSpecificationModelInner {
    Request(RequestDirSpecificationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DirConfigurationModel {
    pub metadata: SpecificationMetadata,
    // pub inner: DirSpecificationModelInner,
}
