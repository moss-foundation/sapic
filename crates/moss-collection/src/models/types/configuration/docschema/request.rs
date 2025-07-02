use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::types::configuration::docschema::{
    HeaderName, RawHeaderParameter, RawMetadata, UrlParts,
};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct RawItemRequestConfiguration {
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
pub struct RawDirRequestConfiguration {
    pub metadata: Block<RawMetadata>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, RawHeaderParameter>>>,
}

impl RawDirRequestConfiguration {
    pub fn new() -> Self {
        Self {
            metadata: Block::new(RawMetadata { id: Uuid::new_v4() }),
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
    use uuid::Uuid;

    #[test]
    fn test_labeled_block() {
        let config = RawItemRequestConfiguration {
            metadata: Block::new(RawMetadata { id: Uuid::new_v4() }),
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
