use serde::{Deserialize, Serialize};

use crate::configuration::SpecificationMetadata;

pub enum RequestItemSpecificationModel {}

pub enum ItemSpecificationModelInner {
    Request(RequestItemSpecificationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ItemConfigurationModel {
    pub metadata: SpecificationMetadata,
    // pub inner: ItemSpecificationModelInner,
}
