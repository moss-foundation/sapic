use crate::{
    constants::ID_LENGTH,
    models::types::configuration::docschema::{
        HeaderName, RawHeaderParameter, RawMetadata, UrlParts,
    },
};
use hcl::ser::{Block, LabeledBlock};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct RawItemEndpointConfiguration {
    pub metadata: Block<RawMetadata>,

    #[serde(flatten)]
    pub url: Block<UrlParts>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, RawHeaderParameter>>>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct RawDirEndpointConfiguration {
    pub metadata: Block<RawMetadata>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, RawHeaderParameter>>>,
}

impl RawDirEndpointConfiguration {
    pub fn new() -> Self {
        Self {
            metadata: Block::new(RawMetadata {
                id: nanoid::nanoid!(ID_LENGTH),
            }),
            headers: None,
        }
    }
}
