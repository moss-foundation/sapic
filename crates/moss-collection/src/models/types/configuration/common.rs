use hcl::ser::Block;
use serde_json::Number;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{primitives::HttpMethod, types::configuration::docschema::RawMetadata};

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum Expression {
    String(String),
    Variable(String),
    Number(Number),
    Bool(bool),
}

// ########################################################
// ###                      Metadata                    ###
// ########################################################

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ConfigurationMetadata {
    pub id: String,
}

impl Into<Block<RawMetadata>> for ConfigurationMetadata {
    fn into(self) -> Block<RawMetadata> {
        Block::new(RawMetadata { id: self.id.into() })
    }
}

impl From<Block<RawMetadata>> for ConfigurationMetadata {
    fn from(block: Block<RawMetadata>) -> Self {
        let inner = block.into_inner();
        Self {
            id: inner.id.to_string(),
        }
    }
}

// ########################################################
// ###                      HTTP                        ###
// ########################################################

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpRequestParts {
    pub method: HttpMethod,
    // pub headers: Vec<HeaderParamItem>,
}

// impl From<Block<docschema::HttpRequestParts>> for HttpRequestParts {
//     fn from(value: Block<docschema::HttpRequestParts>) -> Self {
//         let inner = value.into_inner();

//         todo!()
//         // Self {
//         //     method: inner.method,
//         // }
//     }
// }

// Query Parameter

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct QueryParamOptions {
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts", optional_fields)]
pub struct QueryParamItem {
    pub key: String,
    pub value: Option<Expression>,
    pub order: Option<isize>,
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: QueryParamOptions,
}

// Path Parameter

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PathParamOptions {
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts", optional_fields)]
pub struct PathParamItem {
    pub key: String,
    pub value: Option<Expression>,
    pub order: Option<isize>,
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: PathParamOptions,
}

// Header Parameter
/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HeaderParamOptions {
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts", optional_fields)]
pub struct HeaderParamItem {
    pub key: String,
    pub value: Option<Expression>,
    pub order: Option<isize>,
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: HeaderParamOptions,
}

// Body
/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "types.ts")]
pub enum RawBodyType {
    Text(String),
    Json(String),
    Html(String),
    Xml(String),
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct FormDataOptions {
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum FormDataValue {
    Text(String),
    File(PathBuf),
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct FormDataItem {
    pub key: String,
    pub value: FormDataValue,
    #[ts(optional)]
    pub order: Option<isize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: FormDataOptions,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UrlEncodedOptions {
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
pub struct UrlEncodedItem {
    pub key: String,
    pub value: String,
    pub order: Option<isize>,
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: UrlEncodedOptions,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestBody {
    Raw(RawBodyType),
    FormData(Vec<FormDataItem>),
    UrlEncoded(Vec<UrlEncodedItem>),
    Binary(String),
}
