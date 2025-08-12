use moss_hcl::Block;
use serde::{Deserialize, Serialize};

use crate::{models::primitives::EntryId, spec::EntryMetadataSpec};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSchemaSpec {
    pub metadata: Block<EntryMetadataSpec>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirSchemaSpec {
    pub metadata: Block<EntryMetadataSpec>,
}

impl DirSchemaSpec {
    pub fn new(id: &EntryId) -> Self {
        Self {
            metadata: Block::new(EntryMetadataSpec { id: id.to_owned() }),
        }
    }
}
