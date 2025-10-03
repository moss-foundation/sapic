use hcl::Expression;
use indexmap::IndexMap;
use moss_hcl::{
    Block, LabeledBlock, deserialize_expression, expression,
    heredoc::serialize_option_string_as_heredoc, serialize_expression,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::models::primitives::{
    EntryClass, EntryId, EntryProtocol, FormDataParamId, HeaderId, PathParamId, QueryParamId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadataSpec {
    pub id: EntryId,
    #[serde(rename = "_class")]
    pub class: EntryClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDetails {
    pub protocol: EntryProtocol,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderParamSpec {
    pub name: String,
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression",
        skip_serializing_if = "expression::is_null"
    )]
    pub value: Expression,
    pub description: Option<String>,
    pub options: HeaderParamSpecOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathParamSpec {
    pub name: String,
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression",
        skip_serializing_if = "expression::is_null"
    )]
    pub value: Expression,
    pub description: Option<String>,
    pub options: PathParamSpecOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParamSpec {
    pub name: String,
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression",
        skip_serializing_if = "expression::is_null"
    )]
    pub value: Expression,
    pub description: Option<String>,
    pub options: QueryParamSpecOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDataParamSpec {
    pub name: String,
    pub text: Option<String>,
    pub description: Option<String>,
    pub options: FormDataParamSpecOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDataParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_string_as_heredoc")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<JsonValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_data: Option<LabeledBlock<IndexMap<FormDataParamId, FormDataParamSpec>>>,
}

impl Default for BodySpec {
    fn default() -> Self {
        Self {
            text: None,
            json: None,
            from_data: None,
        }
    }
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<LabeledBlock<IndexMap<String, BodySpec>>>,
}

impl From<(EntryId, EntryClass)> for EntryModel {
    fn from((id, class): (EntryId, EntryClass)) -> Self {
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
    use serde_json::json;

    use super::*;

    fn test_item() -> EntryModel {
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
                    value: Expression::String("bar".to_string()),
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
                "json".to_string() => BodySpec {
                    json: Some(json!({
                        "text": "text",
                        "array": [1, 2, 3],
                    })),
                    ..Default::default()
                },
                "form-data".to_string() => BodySpec {
                    from_data: Some(LabeledBlock::new(indexmap! {
                        FormDataParamId::new() => FormDataParamSpec {
                            name: "form_data_param1".to_string(),
                            text: Some("text".to_string()),
                            description: None,
                            options: FormDataParamSpecOptions {
                                disabled: false,
                                propagate: true
                            }
                        },
                        FormDataParamId::new() => FormDataParamSpec {
                            name: "form_data_param2".to_string(),
                            text: None,
                            description: None,
                            options: FormDataParamSpecOptions {
                                disabled: false,
                                propagate: true
                            }
                        }
                    })),
                    ..Default::default()
                }
            })),
        };

        let str = hcl::to_string(&model).unwrap();
        println!("{}", str);

        let json = serde_json::to_string(&model).unwrap();
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
