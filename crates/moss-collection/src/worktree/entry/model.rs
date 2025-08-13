use hcl::Expression;
use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock};
use serde::{Deserialize, Serialize};

use crate::models::primitives::{EntryClass, EntryId, EntryProtocol, HeaderId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDetails {
    pub protocol: EntryProtocol,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderParamOptions {
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderParamSpec {
    pub name: String,
    pub value: Expression,
    pub disabled: bool,
    pub description: String,
    pub options: HeaderParamOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadataSpec {
    pub id: EntryId,
    #[serde(rename = "_class")]
    pub class: EntryClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryModel {
    pub metadata: Block<EntryMetadataSpec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Block<UrlDetails>>,

    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<LabeledBlock<IndexMap<HeaderId, HeaderParamSpec>>>,
}

impl From<(EntryId, EntryClass)> for EntryModel {
    fn from((id, class): (EntryId, EntryClass)) -> Self {
        Self {
            metadata: Block::new(EntryMetadataSpec { id, class }),
            url: None,
            headers: None,
        }
    }
}

impl EntryModel {
    pub fn id(&self) -> EntryId {
        self.metadata.id.clone()
    }

    pub fn class(&self) -> EntryClass {
        self.metadata.class.clone()
    }

    pub fn protocol(&self) -> Option<EntryProtocol> {
        self.url.as_ref().map(|url| url.protocol.clone())
    }
}

#[cfg(test)]
mod tests {
    use hcl::{Expression as HclExpression, ser::LabeledBlock};
    use indexmap::indexmap;

    use super::*;

    #[test]
    fn test_item() {
        let model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: EntryId::new(),
                class: EntryClass::Request,
            }),
            url: Some(Block::new(UrlDetails {
                protocol: EntryProtocol::Get,
                raw: "https://example.com".to_string(),
            })),
            headers: Some(LabeledBlock::new(indexmap! {
                    HeaderId::new() => HeaderParamSpec {
                        name: "Content-Type".to_string(),
                        value: HclExpression::String("application/json".to_string()),
                        disabled: false,
                        description: "The content type of the request".to_string(),
                        options: HeaderParamOptions { propagate: true },
                    },
                    HeaderId::new() => HeaderParamSpec {
                        name: "Accept".to_string(),
                        value: HclExpression::String("application/json, application/xml".to_string()),
                        disabled: false,
                        description: "The accept type of the request".to_string(),
                        options: HeaderParamOptions { propagate: true },
                }
            })),
        };

        let str = hcl::to_string(&model).unwrap();
        println!("{}", str);

        let json = serde_json::to_string(&model).unwrap();
        println!("{}", json);

        let new = hcl::from_str::<EntryModel>(&str).unwrap();

        println!("{:?}", new);
    }
}
