use crate::{constants::ID_LENGTH, models::types::configuration::docschema::RawMetadata};
use moss_hcl::Block;
use serde::{Deserialize, Serialize};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct RawItemComponentConfiguration {
    pub metadata: Block<RawMetadata>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct RawDirComponentConfiguration {
    pub metadata: Block<RawMetadata>,
}

impl RawDirComponentConfiguration {
    pub fn new() -> Self {
        Self {
            metadata: Block::new(RawMetadata {
                id: nanoid::nanoid!(ID_LENGTH),
            }),
        }
    }
}
