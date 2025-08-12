use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock};
use serde::{Deserialize, Serialize};

use crate::{
    models::primitives::EntryId,
    spec::{EntryMetadataSpec, HeaderName, HeaderParamSpec, UrlParts},
};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRequestSpec {
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
pub struct DirRequestSpec {
    pub metadata: Block<EntryMetadataSpec>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderName, HeaderParamSpec>>>,
}

impl DirRequestSpec {
    pub fn new(id: &EntryId) -> Self {
        Self {
            metadata: Block::new(EntryMetadataSpec { id: id.to_owned() }),
            headers: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use hcl::{Expression as HclExpression, ser::LabeledBlock};
    use indexmap::indexmap;

    use crate::spec::{HeaderParamOptions, HeaderParamSpec, Object, UrlDetails};

    use super::*;

    #[test]
    fn test_labeled_block() {
        let config = ItemRequestSpec {
            metadata: Block::new(EntryMetadataSpec { id: EntryId::new() }),
            url: Block::new(UrlParts::Get(Block::new(UrlDetails {
                raw: "https://example.com".to_string(),
            }))),
            headers: Some(LabeledBlock::new(indexmap! {
                    "Content-Type".to_string() => HeaderParamSpec {
                        value: HclExpression::String("application/json".to_string()),
                        disabled: false,
                        description: "The content type of the request".to_string(),
                        options: Object::new(HeaderParamOptions { propagate: true }),
                    },
                    "Accept".to_string() => HeaderParamSpec {
                        value: HclExpression::String("application/json, application/xml".to_string()),
                        disabled: false,
                        description: "The accept type of the request".to_string(),
                    options: Object::new(HeaderParamOptions { propagate: true }),
                }
            })),
        };

        let str = hcl::to_string(&config).unwrap();
        println!("{}", str);

        let new = hcl::from_str::<ItemRequestSpec>(&str).unwrap();

        println!("{:?}", new);
    }
}
