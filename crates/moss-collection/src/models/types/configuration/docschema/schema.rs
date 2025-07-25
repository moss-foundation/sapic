use moss_hcl::Block;
use serde::{Deserialize, Serialize};

use crate::models::{primitives::EntryId, types::configuration::docschema::RawMetadata};

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
    pub fn new(id: &EntryId) -> Self {
        Self {
            metadata: Block::new(RawMetadata { id: id.to_owned() }),
        }
    }
}
