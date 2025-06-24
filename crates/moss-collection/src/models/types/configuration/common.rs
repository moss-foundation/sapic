use anyhow::Result;
use serde_json::{Number, Value};
use std::{path::PathBuf, str::FromStr};

use hcl::Block;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::models::primitives::{EntryKind, HttpMethod};

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

impl ConfigurationMetadata {
    pub fn to_hcl(&self) -> Block {
        Block::builder("metadata")
            .add_attribute(("id", self.id.to_string()))
            .build()
    }

    pub fn from_hcl(block: &Block) -> Result<Self> {
        let mut id = None;

        for attr in block.body.attributes() {
            if attr.key.as_str() == "id" {
                if let hcl::Expression::String(id_str) = &attr.expr {
                    id = Some(Uuid::parse_str(id_str)?);
                    break;
                }
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| anyhow::anyhow!("Missing id in metadata"))?,
        })
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

impl HeaderParamItem {
    pub fn from_hcl(block: &Block) -> Result<Self> {
        // let mut key = None;
        // let mut value = None;

        // for attr in block.body.attributes() {
        //     if attr.key.as_str() == "key" {
        //         if let HclExpression::String(key_str) = &attr.expr {
        //             key = Some(key_str.to_string());
        //         }
        //     }

        //     if attr.key.as_str() == "value" {
        //         if let HclExpression::String(value_str) = &attr.expr {
        //             value = Some(value_str.to_string());
        //         }
        //     }
        // }

        todo!()
    }
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
