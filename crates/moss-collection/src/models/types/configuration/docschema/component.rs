use hcl::ser::Block;
use serde::{Deserialize, Serialize};

use crate::models::types::configuration::docschema::Metadata;

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
struct ItemComponentConfiguration {
    pub metadata: Block<Metadata>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
struct DirComponentConfiguration {
    pub metadata: Block<Metadata>,
}
