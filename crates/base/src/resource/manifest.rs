pub mod http;

use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock};
use serde::{Deserialize, Serialize};

use crate::resource::{manifest::http::*, types::*};

//
// Resource Metadata
//

#[derive(Debug, Clone, Serialize, Deserialize)]
// TODO: should be renamed to ResourceMetadataSpec
pub struct EntryMetadataSpec {
    pub id: ResourceId,
    #[serde(rename = "_class")]
    pub class: ResourceClass,
}

//
// Resource Manifest
//

#[derive(Debug, Clone, Serialize, Deserialize)]
// TODO: should be renamed to ResourceManifest
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<LabeledBlock<IndexMap<BodyKind, BodySpec>>>,
}

impl From<(ResourceId, ResourceClass)> for EntryModel {
    fn from((id, class): (ResourceId, ResourceClass)) -> Self {
        Self {
            metadata: Block::new(EntryMetadataSpec { id, class }),
            url: None,
            headers: None,
            query_params: None,
            path_params: None,
            body: None,
        }
    }
}

impl EntryModel {
    pub fn id(&self) -> ResourceId {
        self.metadata.id.clone()
    }

    pub fn class(&self) -> ResourceClass {
        self.metadata.class.clone()
    }

    pub fn protocol(&self) -> Option<ResourceProtocol> {
        self.url.as_ref().map(|url| url.protocol.clone())
    }

    pub fn url(&self) -> Option<String> {
        self.url.as_ref().map(|url| url.raw.clone())
    }

    pub fn body_kind(&self) -> Option<BodyKind> {
        if let Some(body) = self.body.as_ref() {
            body.iter().map(|(kind, _)| *kind).next()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use hcl::{Expression as HclExpression, ser::LabeledBlock};
    use indexmap::indexmap;
    use serde_json::json;
    use std::path::PathBuf;

    use super::*;

    fn test_item() -> EntryModel {
        let model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: ResourceId::new(),
                class: ResourceClass::Endpoint,
            }),
            url: Some(Block::new(UrlDetails {
                protocol: ResourceProtocol::Get,
                raw: "https://example.com".to_string(),
            })),
            headers: Some(LabeledBlock::new(indexmap! {
                    HeaderId::new() => HeaderParamSpec {
                        name: "Content-Type".to_string(),
                        value: HclExpression::String("application/json".to_string()),
                        description: Some("The content type of the request".to_string()),
                        options: HeaderParamSpecOptions {
                            disabled: false,
                            propagate: true
                        },
                    },
                    HeaderId::new() => HeaderParamSpec {
                        name: "Accept".to_string(),
                        value: HclExpression::String("application/json, application/xml".to_string()),
                        description: Some("The accept type of the request".to_string()),
                        options: HeaderParamSpecOptions {
                            disabled: false,
                            propagate: true
                    },
                }
            })),
            path_params: Some(LabeledBlock::new(indexmap! {
                PathParamId::new() => PathParamSpec {
                    name: "path_param1".to_string(),
                    value: HclExpression::String("bar".to_string()),
                    description: None,
                    options: PathParamSpecOptions {
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
                    options: QueryParamSpecOptions {
                        disabled: false,
                        propagate: true
                    }
                }
            })),
            body: Some(LabeledBlock::new(indexmap! {
                BodyKind::Text => BodySpec {
                    text: Some("foo".to_string()),
                    ..Default::default()
                },
                BodyKind::Json => BodySpec {
                    json: Some(json!({
                        "text": "text",
                        "array": [1, 2, 3],
                    })),
                    ..Default::default()
                },
                BodyKind::Xml => BodySpec {
                    xml: Some(r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string()),
                    ..Default::default()
                },
                BodyKind::Binary => BodySpec {
                    binary: Some(PathBuf::from("foo/bar.txt")),
                    ..Default::default()
                },
                BodyKind::Urlencoded => BodySpec {
                    urlencoded: Some(LabeledBlock::new(indexmap! {
                        UrlencodedParamId::new() => UrlencodedParamSpec {
                            name: "urlencoded_param1".to_string(),
                            value: HclExpression::String("value1".to_string()),
                            description: None,
                            options: UrlencodedParamSpecOptions {
                                disabled: false,
                                propagate: false,
                            }
                        }
                    })),
                    ..Default::default()
                },
                BodyKind::FormData => BodySpec {
                    form_data: Some(LabeledBlock::new(indexmap! {
                        FormDataParamId::new() => FormDataParamSpec {
                            name: "form_data_param1".to_string(),
                            value: HclExpression::String("value1".to_string()),
                            description: None,
                            options: FormDataParamSpecOptions {
                                disabled: false,
                                propagate: true
                            }
                        }
                    }))
                    ,
                    ..Default::default()
                }
            })),
        };

        let str = hcl::to_string(&model).unwrap();
        println!("{}", str);

        let json = serde_json::to_string_pretty(&model).unwrap();
        println!("{}", json);

        let model = hcl::from_str::<EntryModel>(&str).unwrap();

        model
    }

    #[test]
    fn test_edit() {
        let model = test_item();
        let model_string = hcl::to_string(&model).unwrap();

        let model = hcl::from_str::<EntryModel>(&model_string).unwrap();
        dbg!(&model);
    }
}
