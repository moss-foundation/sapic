use hcl::Expression;
use indexmap::IndexMap;
use moss_hcl::{
    LabeledBlock, deserialize_expression, expression, heredoc::serialize_option_string_as_heredoc,
    serialize_expression,
};
use moss_id_macro::ids;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as JsonValue;
use std::path::PathBuf;

use crate::resource::types::*;

ids!([
    UrlencodedParamId,
    FormDataParamId,
    HeaderId,
    PathParamId,
    QueryParamId
]);

//
// URL Details
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDetails {
    pub protocol: ResourceProtocol,
    pub raw: String,
}

//
// URLencoded Param
//

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

//
// Header Param
//

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

//
// Path Param
//

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

//
// Query Param
//

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

//
// Form Data Param
//

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

//
// Body
//

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
