use hcl::ser::Block;
use serde::{Deserialize, Serialize};

use crate::models::types::configuration::docschema::Metadata;

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
struct ItemSchemaConfiguration {
    pub metadata: Block<Metadata>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
struct DirSchemaConfiguration {
    pub metadata: Block<Metadata>,
}
