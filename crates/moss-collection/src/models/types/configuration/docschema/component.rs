use crate::models::types::configuration::docschema::RawMetadata;
use moss_common::NanoId;
use moss_hcl::Block;
use serde::{Deserialize, Serialize};

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
    pub fn new(id: &NanoId) -> Self {
        Self {
            metadata: Block::new(RawMetadata { id: id.to_owned() }),
        }
    }
}
