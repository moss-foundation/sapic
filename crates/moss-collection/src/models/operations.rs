use moss_common::leased_slotmap::ResourceKey;
use serde::Serialize;
use std::path::{Path, PathBuf};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::models::types::{
    HeaderParamItem, HttpMethod, PathParamItem, QueryParamItem, RequestBody, RequestNodeInfo,
};

/// All the path and file names passed in the input should be unencoded.
/// For example, a name of "workspace.name" will be encoded as "workspace%2Ename"
/// The frontend should simply use the name and path used in the user's original input

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum CreateRequestProtocolSpecificPayload {
    Http {
        method: HttpMethod,
        query_params: Vec<QueryParamItem>,
        path_params: Vec<PathParamItem>,
        headers: Vec<HeaderParamItem>,
        body: Option<RequestBody>,
    },
}

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateRequestInput {
    #[validate(length(min = 1))]
    pub name: String,
    #[ts(optional)]
    pub relative_path: Option<PathBuf>,
    #[ts(optional)]
    pub url: Option<String>,
    #[ts(optional)]
    pub payload: Option<CreateRequestProtocolSpecificPayload>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateRequestOutput {
    pub key: ResourceKey,
}

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RenameRequestInput {
    pub key: ResourceKey,
    #[validate(length(min = 1))]
    pub new_name: String,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteRequestInput {
    pub key: ResourceKey,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListRequestsOutput(pub Vec<RequestNodeInfo>);

#[derive(Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateRequestGroupInput {
    #[validate(custom(function = "validate_path"))]
    pub path: PathBuf, // TODO: spec payload
}

// TODO: More sophisticated path validation
// Right now, we will encode each part of the path in the input
// This will prevent special characters from causing confusion

fn validate_path(path: &Path) -> Result<(), ValidationError> {
    // Check the path ends with a non-empty folder name
    if path.file_name().unwrap_or_default().is_empty() {
        Err(ValidationError::new(""))
    } else {
        Ok(())
    }
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateRequestGroupOutput {
    pub key: ResourceKey,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteRequestGroupInput {
    pub key: ResourceKey,
}

#[derive(Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RenameRequestGroupInput {
    pub key: ResourceKey,
    #[validate(length(min = 1))]
    pub new_name: String,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RenameRequestGroupOutput {
    pub key: ResourceKey,
    pub affected_items: Vec<ResourceKey>,
}
