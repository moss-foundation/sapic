use hcl::ser::{Block, LabeledBlock};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::models::{
    primitives::{EntryId, EntryProtocol},
    types::configuration::docschema::{HeaderName, RawHeaderParameter, RawMetadata, UrlParts},
};
// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawItemEndpointConfiguration {
    pub metadata: Block<RawMetadata>,

    #[serde(flatten)]
    pub url: Block<UrlParts>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, RawHeaderParameter>>>,
}

impl RawItemEndpointConfiguration {
    pub fn change_protocol(&mut self, protocol: EntryProtocol) {
        let details = self.url.details().clone();
        let new_url = match protocol {
            EntryProtocol::Get => UrlParts::Get(Block::new(details)),
            EntryProtocol::Post => UrlParts::Post(Block::new(details)),
            EntryProtocol::Put => UrlParts::Put(Block::new(details)),
            EntryProtocol::Delete => UrlParts::Delete(Block::new(details)),
            EntryProtocol::WebSocket => unimplemented!(),
            EntryProtocol::Graphql => unimplemented!(),
            EntryProtocol::Grpc => unimplemented!(),
        };

        self.url = Block::new(new_url);
    }
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawDirEndpointConfiguration {
    pub metadata: Block<RawMetadata>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, RawHeaderParameter>>>,
}

impl RawDirEndpointConfiguration {
    pub fn new(id: &EntryId) -> Self {
        Self {
            metadata: Block::new(RawMetadata { id: id.to_owned() }),
            headers: None,
        }
    }
}
