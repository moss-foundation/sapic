use hcl::Expression;
use indexmap::IndexMap;
use moss_hcl::{
    Block, LabeledBlock, deserialize_expression, expression,
    heredoc::serialize_option_string_as_heredoc, serialize_expression,
};
use sapic_base::resource::types::primitives::ResourceId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as JsonValue;
use std::path::PathBuf;

use crate::models::primitives::{
    FormDataParamId, HeaderId, PathParamId, QueryParamId, ResourceClass, ResourceProtocol,
    UrlencodedParamId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadataSpec {
    pub id: ResourceId,
    #[serde(rename = "_class")]
    pub class: ResourceClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDetails {
    pub protocol: ResourceProtocol,
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
pub struct UrlencodedParamSpec {
    pub name: String,
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression",
        skip_serializing_if = "expression::is_null"
    )]
    pub value: Expression,
    pub description: Option<String>,
    pub options: UrlencodedParamSpecOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlencodedParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDataParamSpec {
    pub name: String,
    // multipart/form-data can contain both data and files
    // We will use functions to support files
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression",
        skip_serializing_if = "expression::is_null"
    )]
    pub value: Expression,
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

    // TODO: Find a way to fully support xml
    // Currently there isn't a good counterpart to serde_json::Value for xml
    // `xmltree::Element` will silently discard extra root nodes instead of raising an error
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_string_as_heredoc")]
    pub xml: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlencoded: Option<LabeledBlock<IndexMap<UrlencodedParamId, UrlencodedParamSpec>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_data: Option<LabeledBlock<IndexMap<FormDataParamId, FormDataParamSpec>>>,
}

impl Default for BodySpec {
    fn default() -> Self {
        Self {
            text: None,
            json: None,
            xml: None,
            binary: None,
            urlencoded: None,
            form_data: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyKind {
    Text,
    Json,
    Xml,
    Binary,
    Urlencoded,
    FormData,
}

impl Serialize for BodyKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            BodyKind::Text => serializer.serialize_str("text"),
            BodyKind::Json => serializer.serialize_str("json"),
            BodyKind::Xml => serializer.serialize_str("xml"),
            BodyKind::Binary => serializer.serialize_str("binary"),
            BodyKind::Urlencoded => serializer.serialize_str("x-www-form-urlencoded"),
            BodyKind::FormData => serializer.serialize_str("form-data"),
        }
    }
}

impl<'de> Deserialize<'de> for BodyKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let typ = String::deserialize(deserializer)?;
        match typ.as_str() {
            "text" => Ok(BodyKind::Text),
            "json" => Ok(BodyKind::Json),
            "xml" => Ok(BodyKind::Xml),
            "binary" => Ok(BodyKind::Binary),
            "x-www-form-urlencoded" => Ok(BodyKind::Urlencoded),
            "form-data" => Ok(BodyKind::FormData),
            _ => Err(serde::de::Error::custom(format!(
                "unknown body kind: {}",
                typ
            ))),
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
                            value: Expression::String("value1".to_string()),
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
                            value: Expression::String("value1".to_string()),
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
