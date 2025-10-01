use crate::models::primitives::{
    EntryClass, EntryId, EntryProtocol, FormDataParamId, HeaderId, PathParamId, QueryParamId,
};
use hcl::Expression;
use indexmap::IndexMap;
use moss_hcl::{
    Block, LabeledBlock, deserialize_expression, expression, heredoc::*, serialize_expression,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::path::PathBuf;

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
pub enum FormDataParamValue {
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression"
    )]
    #[serde(rename = "text")]
    Text(Expression),

    #[serde(rename = "path")]
    Binary(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDataParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDataParamSpec {
    pub name: String,
    // TODO: Handling both text value and file upload
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/POST
    pub value: FormDataParamValue,
    pub description: Option<String>,
    pub options: FormDataParamSpecOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum BodyValue {
    #[serde(serialize_with = "serialize_string_as_heredoc")]
    Text(String),

    #[serde(serialize_with = "serialize_jsonvalue_as_heredoc")]
    Json(JsonValue),

    FormData(LabeledBlock<IndexMap<FormDataParamId, FormDataParamSpec>>),

    Binary(PathBuf),
}

// TODO:
// I guess it's better to name it ResourceModel ?
// Ticket: https://mossland.atlassian.net/browse/SAPIC-533
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

    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<LabeledBlock<IndexMap<String, BodyValue>>>,
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
                "text".to_string() => BodyValue::Text("body".to_string()),
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
