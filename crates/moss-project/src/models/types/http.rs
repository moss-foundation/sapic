use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;
use validator::Validate;

use crate::models::primitives::{HeaderId, PathParamId, QueryParamId};

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct QueryParamOptions {
    pub disabled: bool,
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
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct HeaderParamOptions {
    pub disabled: bool,
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
