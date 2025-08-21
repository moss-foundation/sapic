pub mod http;

use http::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::{
    dirs,
    models::primitives::{
        EntryId, EntryProtocol, FrontendEntryPath, HeaderId, PathParamId, QueryParamId,
    },
};

/// @category Type
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfo {
    pub id: String,
    pub name: String,

    /// Determines the display position of this entry among others in the same group.
    /// Entries are sorted in ascending order; lower values appear before higher ones.
    /// Negative values are allowed and will be placed before positive values.
    /// If multiple entries have the same order, they are sorted alphabetically.
    /// If not specified, the entry appears last and is sorted alphabetically
    /// among unspecified items.
    pub order: Option<isize>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CreateItemEntryParams {
    #[validate(custom(function = "validate_create_entry_input_path"))]
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: String,
    pub order: isize,

    pub protocol: Option<EntryProtocol>,

    pub query_params: Vec<AddQueryParamParams>,
    pub path_params: Vec<AddPathParamParams>,
    pub headers: Vec<AddHeaderParams>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CreateDirEntryParams {
    #[validate(custom(function = "validate_create_entry_input_path"))]
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: String,
    pub order: isize,

    pub headers: Vec<AddHeaderParams>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateItemEntryParams {
    #[ts(as = "String")]
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

    pub query_params_to_add: Vec<AddQueryParamParams>,
    pub query_params_to_update: Vec<UpdateQueryParamParams>,
    pub query_params_to_remove: Vec<QueryParamId>,

    pub path_params_to_add: Vec<AddPathParamParams>,
    pub path_params_to_update: Vec<UpdatePathParamParams>,
    pub path_params_to_remove: Vec<PathParamId>,

    pub headers_to_add: Vec<AddHeaderParams>,
    pub headers_to_update: Vec<UpdateHeaderParams>,
    pub headers_to_remove: Vec<HeaderId>,
}

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateDirEntryParams {
    #[ts(as = "String")]
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
    #[ts(as = "String")]
    pub id: EntryId,

    pub path: FrontendEntryPath,
}

/// @category Type
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AfterUpdateItemEntryDescription {
    #[ts(as = "String")]
    pub id: EntryId,

    pub path: FrontendEntryPath,
}

// Check that input path begins with a valid top folder
// such as requests, endpoints, etc.
pub(super) fn validate_create_entry_input_path(path: &Path) -> Result<(), ValidationError> {
    for folder in [
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ] {
        if path.starts_with(folder) {
            return Ok(());
        }
    }
    Err(ValidationError::new(
        "The input path does not start with a valid top folder",
    ))
}
