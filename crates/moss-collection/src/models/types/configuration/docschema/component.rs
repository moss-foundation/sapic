use moss_hcl::Block;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::types::configuration::docschema::RawMetadata;

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawItemComponentConfiguration {
    pub metadata: Block<RawMetadata>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawDirComponentConfiguration {
    pub metadata: Block<RawMetadata>,
}

impl RawDirComponentConfiguration {
    pub fn new(id: Uuid) -> Self {
        Self {
            metadata: Block::new(RawMetadata { id }),
        }
    }
}
