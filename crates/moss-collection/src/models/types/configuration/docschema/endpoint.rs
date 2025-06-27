use hcl::ser::{Block, LabeledBlock};
use indexmap::{IndexMap, indexmap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::types::configuration::docschema::{
    HeaderName, RawHeaderParameter, RawMetadata, UrlParts,
};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct RawItemEndpointConfiguration {
    pub metadata: Block<RawMetadata>,

    #[serde(flatten)]
    pub url: Block<UrlParts>,

    #[serde(rename = "header")]
    pub headers: LabeledBlock<IndexMap<HeaderName, RawHeaderParameter>>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct RawDirEndpointConfiguration {
    pub metadata: Block<RawMetadata>,

    #[serde(rename = "header")]
    pub headers: LabeledBlock<IndexMap<HeaderName, RawHeaderParameter>>,
}

impl RawDirEndpointConfiguration {
    pub fn new() -> Self {
        Self {
            metadata: Block::new(RawMetadata { id: Uuid::new_v4() }),
            headers: LabeledBlock::new(indexmap! {}),
        }
    }
}
