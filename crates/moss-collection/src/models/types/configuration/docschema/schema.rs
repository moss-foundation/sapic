use moss_hcl::Block;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::types::configuration::docschema::RawMetadata;

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawItemSchemaConfiguration {
    pub metadata: Block<RawMetadata>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawDirSchemaConfiguration {
    pub metadata: Block<RawMetadata>,
}

impl RawDirSchemaConfiguration {
    pub fn new(id: Uuid) -> Self {
        Self {
            metadata: Block::new(RawMetadata { id }),
        }
    }
}
