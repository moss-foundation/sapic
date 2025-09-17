use hcl::Expression;
use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock};
use serde::{Deserialize, Serialize};

use crate::models::primitives::{
    EntryClass, EntryId, EntryProtocol, HeaderId, PathParamId, QueryParamId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDetails {
    pub protocol: EntryProtocol,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderParamOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderParamSpec {
    pub name: String,
    pub value: Expression,
    pub description: Option<String>,
    pub options: HeaderParamOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParamOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParamSpec {
    pub name: String,
    pub value: Expression,
    pub description: Option<String>,
    pub options: QueryParamOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathParamOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathParamSpec {
    pub name: String,
    pub value: Expression,
    pub description: Option<String>,
    pub options: PathParamOptions,
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

    #[serde(rename = "path_param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_params: Option<LabeledBlock<IndexMap<PathParamId, PathParamSpec>>>,

    #[serde(rename = "query_param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<LabeledBlock<IndexMap<QueryParamId, QueryParamSpec>>>,
}

impl From<(EntryId, EntryClass)> for EntryModel {
    fn from((id, class): (EntryId, EntryClass)) -> Self {
        Self {
            metadata: Block::new(EntryMetadataSpec { id, class }),
            url: None,
            headers: None,
            query_params: None,
            path_params: None,
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
                class: EntryClass::Endpoint,
            }),
            url: Some(Block::new(UrlDetails {
                protocol: EntryProtocol::Get,
                raw: "https://example.com".to_string(),
            })),
            headers: Some(LabeledBlock::new(indexmap! {
                    HeaderId::new() => HeaderParamSpec {
                        name: "Content-Type".to_string(),
                        value: HclExpression::String("application/json".to_string()),
                        description: Some("The content type of the request".to_string()),
                        options: HeaderParamOptions {
                            disabled: false,
                            propagate: true
                        },
                    },
                    HeaderId::new() => HeaderParamSpec {
                        name: "Accept".to_string(),
                        value: HclExpression::String("application/json, application/xml".to_string()),
                        description: Some("The accept type of the request".to_string()),
                        options: HeaderParamOptions {
                            disabled: false,
                            propagate: true
                    },
                }
            })),
            path_params: Some(LabeledBlock::new(indexmap! {
                PathParamId::new() => PathParamSpec {
                    name: "path_param1".to_string(),
                    value: Expression::String("bar".to_string()),
                    description: None,
                    options: PathParamOptions {
                        disabled: false,
                        propagate: true,
                    },
                }
            })),
            query_params: Some(LabeledBlock::new(indexmap! {
                QueryParamId::new() => QueryParamSpec {
                    name: "query_param1".to_string(),
                    value: HclExpression::String("foo".to_string()),
                    description: None,
                    options: QueryParamOptions {
                        disabled: false,
                        propagate: true
                    }
                }
            })),
        };

        let str = hcl::to_string(&model).unwrap();
        println!("{}", str);

        let json = serde_json::to_string(&model).unwrap();
        println!("{}", json);

        let new = hcl::from_str::<EntryModel>(&str).unwrap();

        println!("{:#?}", new);
    }
}
