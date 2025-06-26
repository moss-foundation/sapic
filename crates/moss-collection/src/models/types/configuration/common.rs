use anyhow::Result;
use hcl::ser::Block;
use serde_json::{Number, Value};
use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::models::{
    primitives::{EntryKind, HttpMethod},
    types::configuration::docschema::{self, Metadata},
};

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum Expression {
    Null,
    String(String),
    Variable(String),
    Number(Number),
    Bool(bool),
}

// ########################################################
// ###                      Metadata                    ###
// ########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ConfigurationMetadata {
    pub id: Uuid,
}

impl From<Block<Metadata>> for ConfigurationMetadata {
    fn from(block: Block<Metadata>) -> Self {
        let inner = block.into_inner();
        Self { id: inner.id }
    }
}

// ########################################################
// ###                      HTTP                        ###
// ########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpRequestParts {
    pub method: HttpMethod,
    // pub headers: Vec<HeaderParamItem>,
}

impl From<Block<docschema::HttpRequestParts>> for HttpRequestParts {
    fn from(value: Block<docschema::HttpRequestParts>) -> Self {
        let inner = value.into_inner();
        Self {
            method: inner.method,
        }
    }
}

// Query Parameter

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct QueryParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct QueryParamItem {
    pub key: String,
    pub value: Expression,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: QueryParamOptions,
}

// Path Parameter

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PathParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PathParamItem {
    pub key: String,
    pub value: Expression,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: PathParamOptions,
}

// Header Parameter

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HeaderParamOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HeaderParamItem {
    pub key: String,
    pub value: Expression,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: HeaderParamOptions,
}

// Body

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "types.ts")]
pub enum RawBodyType {
    Text(String),
    Json(String),
    Html(String),
    Xml(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct FormDataOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum FormDataValue {
    Text(String),
    File(PathBuf),
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct FormDataItem {
    pub key: String,
    pub value: FormDataValue,
    #[ts(optional)]
    pub order: Option<usize>,
    #[ts(optional)]
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: FormDataOptions,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UrlEncodedOptions {
    pub propagate: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
pub struct UrlEncodedItem {
    pub key: String,
    pub value: String,
    pub order: Option<usize>,
    pub desc: Option<String>,
    pub disabled: bool,
    pub options: UrlEncodedOptions,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestBody {
    Raw(RawBodyType),
    FormData(Vec<FormDataItem>),
    UrlEncoded(Vec<UrlEncodedItem>),
    Binary(String),
}
