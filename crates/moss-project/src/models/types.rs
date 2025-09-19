pub mod http;

use http::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::path::{Path, PathBuf};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::models::primitives::{
    EntryClass, EntryId, EntryProtocol, FrontendEntryPath, HeaderId, PathParamId, QueryParamId,
};

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CreateItemEntryParams {
    #[validate(custom(function = "validate_create_entry_input_path"))]
    pub path: PathBuf,
    pub class: EntryClass,

    #[validate(length(min = 1))]
    pub name: String,
    pub order: isize,

    // TODO: url
    pub protocol: Option<EntryProtocol>,

    pub headers: Vec<AddHeaderParams>,
    pub path_params: Vec<AddPathParamParams>,
    pub query_params: Vec<AddQueryParamParams>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CreateDirEntryParams {
    #[validate(custom(function = "validate_create_entry_input_path"))]
    pub path: PathBuf,
    pub class: EntryClass,

    #[validate(length(min = 1))]
    pub name: String,
    pub order: isize,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateItemEntryParams {
    pub id: EntryId,

    /// If provided, the entry will move to the new path
    /// For example, if the new path is "requests/folder/", the name is "entry"
    /// The new relative path of the entry folder will be "requests/folder/entry"
    pub path: Option<PathBuf>,

    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub order: Option<isize>,
    pub expanded: Option<bool>,

    pub protocol: Option<EntryProtocol>,

    pub headers_to_add: Vec<AddHeaderParams>,
    pub headers_to_update: Vec<UpdateHeaderParams>,
    pub headers_to_remove: Vec<HeaderId>,

    pub path_params_to_add: Vec<AddPathParamParams>,
    pub path_params_to_update: Vec<UpdatePathParamParams>,
    pub path_params_to_remove: Vec<PathParamId>,

    pub query_params_to_add: Vec<AddQueryParamParams>,
    pub query_params_to_update: Vec<UpdateQueryParamParams>,
    pub query_params_to_remove: Vec<QueryParamId>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateDirEntryParams {
    pub id: EntryId,

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
pub struct AfterUpdateDirEntryDescription {
    pub id: EntryId,

    pub path: FrontendEntryPath,
}

/// @category Type
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AfterUpdateItemEntryDescription {
    pub id: EntryId,

    pub path: FrontendEntryPath,
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

// Check that input path begins with a valid top folder
// such as requests, endpoints, etc.
pub(super) fn validate_create_entry_input_path(path: &Path) -> Result<(), ValidationError> {
    if path.is_absolute() {
        return Err(ValidationError::new("the input path must be relative"));
    }

    Ok(())
}
