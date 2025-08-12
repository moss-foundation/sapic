use hcl::ser::{Block, LabeledBlock};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    models::primitives::EntryId,
    spec::{EntryMetadataSpec, HeaderName, HeaderParamSpec, UrlParts},
};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemEndpointSpec {
    pub metadata: Block<EntryMetadataSpec>,

    #[serde(flatten)]
    pub url: Block<UrlParts>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, HeaderParamSpec>>>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirEndpointSpec {
    pub metadata: Block<EntryMetadataSpec>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, HeaderParamSpec>>>,
}

impl DirEndpointSpec {
    pub fn new(id: &EntryId) -> Self {
        Self {
            metadata: Block::new(EntryMetadataSpec { id: id.to_owned() }),
            headers: None,
        }
    }
}
