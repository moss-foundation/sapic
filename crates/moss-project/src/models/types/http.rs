use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

use crate::models::primitives::{
    FormDataParamId, HeaderId, PathParamId, QueryParamId, UrlencodedParamId,
};

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct HeaderParamOptions {
    pub disabled: bool,
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct AddHeaderParams {
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub order: isize,
    pub desc: Option<String>,
    pub options: HeaderParamOptions,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHeaderParamOptions {
    pub disabled: Option<bool>,
    pub propagate: Option<bool>,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHeaderParams {
    pub id: HeaderId,
    pub name: Option<String>,
    #[ts(optional, type = "ChangeJsonValue")]
    pub value: Option<ChangeJsonValue>,
    pub order: Option<isize>,
    #[ts(optional, type = "ChangeString")]
    pub desc: Option<ChangeString>,
    pub options: Option<HeaderParamOptions>,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct QueryParamOptions {
    pub disabled: bool,
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct AddQueryParamParams {
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub order: isize,
    pub desc: Option<String>,
    pub options: QueryParamOptions,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQueryParamOptions {
    pub disabled: Option<bool>,
    pub propagate: Option<bool>,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQueryParamParams {
    pub id: QueryParamId,
    pub name: Option<String>,
    #[ts(optional, type = "ChangeJsonValue")]
    pub value: Option<ChangeJsonValue>,
    pub order: Option<isize>,
    #[ts(optional, type = "ChangeString")]
    pub desc: Option<ChangeString>,
    pub options: Option<QueryParamOptions>,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct PathParamOptions {
    pub disabled: bool,
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct AddPathParamParams {
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub order: isize,
    pub desc: Option<String>,
    pub options: PathParamOptions,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePathParamOptions {
    pub disabled: Option<bool>,
    pub propagate: Option<bool>,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePathParamParams {
    pub id: PathParamId,
    pub name: Option<String>,
    #[ts(optional, type = "ChangeJsonValue")]
    pub value: Option<ChangeJsonValue>,
    pub order: Option<isize>,
    #[ts(optional, type = "ChangeString")]
    pub desc: Option<ChangeString>,
    pub options: Option<PathParamOptions>,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
pub enum AddBodyParams {
    Text(String),
    Json(#[ts(type = "JsonValue")] JsonValue),
    Xml(String),
    Binary(PathBuf),
    Urlencoded(Vec<AddUrlencodedParamParams>),
    FormData(Vec<AddFormDataParamParams>),
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
pub struct UrlencodedParamOptions {
    pub disabled: bool,
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct AddUrlencodedParamParams {
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub order: isize,
    pub description: Option<String>,
    pub options: UrlencodedParamOptions,
    /// This field should be provided when the frontend switches back to urlencoded body type
    /// We will reuse the old ids to avoid unnecessary changes
    pub id: Option<UrlencodedParamId>,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
pub struct FormDataParamOptions {
    pub disabled: bool,
    pub propagate: bool,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct AddFormDataParamParams {
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub order: isize,
    pub description: Option<String>,
    pub options: FormDataParamOptions,
    /// This field should be provided when the frontend switches back to formdata body type
    /// We will reuse the old ids to avoid unnecessary changes
    pub id: Option<FormDataParamId>,
}
