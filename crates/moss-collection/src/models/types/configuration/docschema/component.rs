use moss_hcl::Block;
use serde::{Deserialize, Serialize};

use crate::models::{primitives::EntryId, types::configuration::docschema::RawMetadata};

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
    pub fn new(id: &EntryId) -> Self {
        Self {
            metadata: Block::new(RawMetadata { id: id.to_owned() }),
        }
    }
}
