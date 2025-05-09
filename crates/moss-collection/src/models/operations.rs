use moss_common::leased_slotmap::ResourceKey;
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use super::types::PathChangeKind;
use crate::models::{
    primitives::EntryId,
    types::{
        HeaderParamItem, HttpMethod, PathParamItem, QueryParamItem, RequestBody, RequestNodeInfo,
    },
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

// TODO: remove this
#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
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

// TODO: remove this
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
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

// Create Request Entry

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateRequestEntryInput {
    #[validate(custom(function = "validate_request_destination"))]
    pub destination: PathBuf,
    #[ts(optional)]
    pub url: Option<String>,
    #[ts(optional)]
    pub payload: Option<CreateRequestProtocolSpecificPayload>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateRequestEntryOutput {
    pub changed_paths: Arc<[(Arc<Path>, EntryId, PathChangeKind)]>,
}

// Create Request Directory Entry

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateRequestDirEntryInput {
    #[validate(custom(function = "validate_request_destination"))]
    pub destination: PathBuf,
}

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateRequestDirEntryOutput {
    pub changed_paths: Arc<[(Arc<Path>, EntryId, PathChangeKind)]>,
}

// Update Request Directory Entry

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateRequestDirEntryInput {
    pub id: EntryId,

    /// A new name for the directory, if provided,
    /// the directory will be renamed to this name.
    #[ts(optional)]
    #[validate(length(min = 1))]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateRequestDirEntryOutput {
    pub changed_paths: Arc<[(Arc<Path>, EntryId, PathChangeKind)]>,
}

// Delete Request Directory Entry

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteRequestDirEntryInput {
    pub id: EntryId,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteRequestDirEntryOutput {
    pub changed_paths: Arc<[(Arc<Path>, EntryId, PathChangeKind)]>,
}

// Stream Entries By Prefixes

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct StreamEntriesByPrefixesInput(pub Vec<&'static str>);

/// Validates the destination path for creating a request entry.
/// Requirements:
/// - Path must not be absolute
/// - First segment must be 'requests'
/// - Path must not contain invalid characters
/// - Path must have at least one component after 'requests'
fn validate_request_destination(destination: &Path) -> Result<(), ValidationError> {
    if destination.is_absolute() {
        return Err(ValidationError::new("Destination path cannot be absolute"));
    }

    if destination.as_os_str().is_empty() {
        return Err(ValidationError::new("Destination path cannot be empty"));
    }

    // Check that the first segment is 'requests'
    let mut components = destination.components();
    let first = components.next();

    match first {
        Some(std::path::Component::Normal(name)) => {
            if name != "requests" {
                return Err(ValidationError::new(
                    "First path segment must be 'requests'",
                ));
            }
        }
        _ => {
            return Err(ValidationError::new(
                "First path segment must be 'requests'",
            ));
        }
    }

    // Ensure there's at least one more component after 'requests'
    if components.next().is_none() {
        return Err(ValidationError::new(
            "Path must contain at least one component after 'requests'",
        ));
    }

    // Check for invalid path characters
    let path_str = destination.to_string_lossy();
    if path_str.contains("..") || path_str.contains("//") {
        return Err(ValidationError::new("Path contains invalid sequences"));
    }

    Ok(())
}
