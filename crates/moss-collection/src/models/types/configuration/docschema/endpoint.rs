use hcl::ser::{Block, LabeledBlock};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::models::types::configuration::docschema::{DirRequestParts, Metadata, Protocol};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
struct ItemEndpointConfiguration {
    pub metadata: Block<Metadata>,
    // #[serde(rename = "request")]
    // pub request_parts: LabeledBlock<IndexMap<Protocol, ItemRequestParts>>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
struct DirEndpointConfiguration {
    pub metadata: Block<Metadata>,

    #[serde(rename = "request")]
    pub request_parts: LabeledBlock<IndexMap<Protocol, DirRequestParts>>,
}
