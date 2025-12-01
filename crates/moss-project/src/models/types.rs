pub mod http;

use http::*;
use sapic_base::resource::types::primitives::ResourceId;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::path::{Path, PathBuf};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::models::primitives::{
    FormDataParamId, FrontendResourcePath, HeaderId, PathParamId, QueryParamId, ResourceClass,
    ResourceProtocol, UrlencodedParamId,
};

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CreateItemResourceParams {
    #[validate(custom(function = "validate_create_resource_input_path"))]
    pub path: PathBuf,
    pub class: ResourceClass,

    #[validate(length(min = 1))]
    pub name: String,
    pub order: isize,

    // TODO: url
    pub protocol: Option<ResourceProtocol>,

    pub headers: Vec<AddHeaderParams>,
    pub path_params: Vec<AddPathParamParams>,
    pub query_params: Vec<AddQueryParamParams>,
    pub body: Option<AddBodyParams>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CreateDirResourceParams {
    #[validate(custom(function = "validate_create_resource_input_path"))]
    pub path: PathBuf,
    pub class: ResourceClass,

    #[validate(length(min = 1))]
    pub name: String,
    pub order: isize,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateItemResourceParams {
    pub id: ResourceId,

    /// If provided, the resource will move to the new path
    /// For example, if the new path is "requests/folder/", the name is "resource"
    /// The new relative path of the resource folder will be "requests/folder/resource"
    pub path: Option<PathBuf>,

    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub order: Option<isize>,
    pub expanded: Option<bool>,

    pub protocol: Option<ResourceProtocol>,

    pub headers_to_add: Vec<AddHeaderParams>,
    pub headers_to_update: Vec<UpdateHeaderParams>,
    pub headers_to_remove: Vec<HeaderId>,

    pub path_params_to_add: Vec<AddPathParamParams>,
    pub path_params_to_update: Vec<UpdatePathParamParams>,
    pub path_params_to_remove: Vec<PathParamId>,

    pub query_params_to_add: Vec<AddQueryParamParams>,
    pub query_params_to_update: Vec<UpdateQueryParamParams>,
    pub query_params_to_remove: Vec<QueryParamId>,

    pub body: Option<UpdateBodyParams>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum UpdateBodyParams {
    Remove,
    Text(String),
    Json(#[ts(type = "JsonValue")] JsonValue),
    Xml(String),
    Binary(PathBuf),
    Urlencoded {
        params_to_add: Vec<AddUrlencodedParamParams>,
        params_to_update: Vec<UpdateUrlencodedParamParams>,
        params_to_remove: Vec<UrlencodedParamId>,
    },
    FormData {
        params_to_add: Vec<AddFormDataParamParams>,
        params_to_update: Vec<UpdateFormDataParamParams>,
        params_to_remove: Vec<FormDataParamId>,
    },
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateDirResourceParams {
    pub id: ResourceId,

    /// If provided, the directory will move to the new path
    /// For example, if the new path is "requests/folder/", the name is "group"
    /// The new relative path of the directory folder will be "requests/folder/group"
    pub path: Option<PathBuf>,

    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub order: Option<isize>,
    pub expanded: Option<bool>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AfterUpdateDirResourceDescription {
    pub id: ResourceId,

    pub path: FrontendResourcePath,
}

/// @category Type
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AfterUpdateItemResourceDescription {
    pub id: ResourceId,

    pub path: FrontendResourcePath,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "types.ts")]
pub enum VcsOperation {
    Commit {
        message: String,
        paths: Vec<PathBuf>,
        push: bool,
    },
    Discard {
        paths: Vec<PathBuf>,
    },
    Push,
    Pull,
    Fetch,
}

/// @category Type
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct HeaderInfo {
    pub id: HeaderId,
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub description: Option<String>,
    pub disabled: bool,
    pub propagate: bool,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct PathParamInfo {
    pub id: PathParamId,
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub description: Option<String>,
    pub disabled: bool,
    pub propagate: bool,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct QueryParamInfo {
    pub id: QueryParamId,
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub description: Option<String>,
    pub disabled: bool,
    pub propagate: bool,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UrlencodedParamInfo {
    pub id: UrlencodedParamId,
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub description: Option<String>,
    pub disabled: bool,
    pub propagate: bool,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct FormDataParamInfo {
    pub id: FormDataParamId,
    pub name: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub description: Option<String>,
    pub disabled: bool,
    pub propagate: bool,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum BodyInfo {
    Text(String),
    Json(#[ts(type = "JsonValue")] JsonValue),
    Xml(String),
    Binary(PathBuf),
    Urlencoded(Vec<UrlencodedParamInfo>),
    FormData(Vec<FormDataParamInfo>),
}

// Check that input path begins with a valid top folder
// such as requests, endpoints, etc.
pub(super) fn validate_create_resource_input_path(path: &Path) -> Result<(), ValidationError> {
    if path.is_absolute() {
        return Err(ValidationError::new("the input path must be relative"));
    }

    Ok(())
}
