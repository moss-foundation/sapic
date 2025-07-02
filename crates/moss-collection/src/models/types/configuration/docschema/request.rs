use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock};
use serde::{Deserialize, Serialize};

use crate::models::{
    primitives::EntryProtocol,
    types::configuration::docschema::{HeaderName, RawHeaderParameter, RawMetadata, UrlParts},
};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawItemRequestConfiguration {
    pub metadata: Block<RawMetadata>,

    #[serde(flatten)]
    pub url: Block<UrlParts>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, RawHeaderParameter>>>,
}

impl RawItemRequestConfiguration {
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
pub struct RawDirRequestConfiguration {
    pub metadata: Block<RawMetadata>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, RawHeaderParameter>>>,
}

impl RawDirRequestConfiguration {
    pub fn new(id: &str) -> Self {
        Self {
            metadata: Block::new(RawMetadata { id: id.to_string() }),
            headers: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::types::configuration::docschema::{
        HeaderParameterOptions, Object, RawHeaderParameter, UrlDetails,
    };

    use super::*;

    use hcl::{Expression as HclExpression, ser::LabeledBlock};
    use indexmap::indexmap;
    use moss_common::nanoid::new_nanoid;

    #[test]
    fn test_labeled_block() {
        let config = RawItemRequestConfiguration {
            metadata: Block::new(RawMetadata { id: new_nanoid() }),
            url: Block::new(UrlParts::Get(Block::new(UrlDetails {
                raw: "https://example.com".to_string(),
            }))),
            headers: Some(LabeledBlock::new(indexmap! {
                    "Content-Type".to_string() => RawHeaderParameter {
                        value: HclExpression::String("application/json".to_string()),
                        disabled: false,
                        description: "The content type of the request".to_string(),
                        options: Object::new(HeaderParameterOptions { propagate: true }),
                    },
                    "Accept".to_string() => RawHeaderParameter {
                        value: HclExpression::String("application/json, application/xml".to_string()),
                        disabled: false,
                        description: "The accept type of the request".to_string(),
                    options: Object::new(HeaderParameterOptions { propagate: true }),
                }
            })),
        };

        let str = hcl::to_string(&config).unwrap();
        println!("{}", str);

        let new = hcl::from_str::<RawItemRequestConfiguration>(&str).unwrap();

        println!("{:?}", new);
    }
}
