use moss_hcl::Block;
use serde::{Deserialize, Serialize};

use crate::models::primitives::EntryId;
use crate::spec::EntryMetadataSpec;

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemComponentSpec {
    pub metadata: Block<EntryMetadataSpec>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirComponentSpec {
    pub metadata: Block<EntryMetadataSpec>,
}

impl DirComponentSpec {
    pub fn new(id: &EntryId) -> Self {
        Self {
            metadata: Block::new(EntryMetadataSpec { id: id.to_owned() }),
        }
    }
}
