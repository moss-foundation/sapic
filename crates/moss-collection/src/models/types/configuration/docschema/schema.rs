use crate::models::types::configuration::docschema::RawMetadata;
use moss_common::NanoId;
use moss_hcl::Block;
use serde::{Deserialize, Serialize};

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
    pub fn new(id: &NanoId) -> Self {
        Self {
            metadata: Block::new(RawMetadata { id: id.to_owned() }),
        }
    }
}
