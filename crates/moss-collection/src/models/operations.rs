use serde::Serialize;
use serde_json::Value as JsonValue;
use std::path::{Path, PathBuf};
use ts_rs::TS;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use super::types::Classification;
use crate::models::{
    primitives::WorktreeDiff,
    types::{
        HeaderParamItem, HttpMethod, PathParamItem, QueryParamItem, RequestBody, RequestProtocol,
    },
};

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
// TODO: Implement new validation logic for destination
pub struct CreateEntryInput {
    pub destination: PathBuf,
    pub classification: Classification,
    #[ts(optional, type = "JsonValue")]
    pub specification: Option<JsonValue>,
    #[ts(optional)]
    pub protocol: Option<RequestProtocol>,
    #[ts(optional)]
    pub order: Option<usize>,
    pub is_dir: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateEntryOutput {
    pub changes: WorktreeDiff,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryInput {
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryOutput {
    pub changes: WorktreeDiff,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEntryInput {
    pub id: Uuid,
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub classification: Option<Classification>,
    #[ts(optional, type = "JsonValue")]
    pub specification: Option<JsonValue>,
    #[ts(optional)]
    pub protocol: Option<RequestProtocol>,
    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEntryOutput {
    pub changes: WorktreeDiff,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum CreateRequestProtocolSpecificPayload {
    Http {
        method: HttpMethod,
        query_params: Vec<QueryParamItem>,
        path_params: Vec<PathParamItem>,
        headers: Vec<HeaderParamItem>,
        #[ts(optional)]
        body: Option<RequestBody>,
    },
}

// Stream Entries By Prefixes

#[derive(Debug, Serialize, TS, Validate)]
#[ts(export, export_to = "operations.ts")]
pub struct StreamWorktreeEntriesInput {
    #[validate(custom(function = "validate_stream_worktree_entries_prefixes"))]
    pub prefixes: Vec<&'static str>,
}

const ALLOWED_PREFIXES: [&str; 4] = ["requests", "endpoints", "components", "schemas"];
fn validate_stream_worktree_entries_prefixes(
    prefixes: &Vec<&'static str>,
) -> Result<(), ValidationError> {
    for prefix in prefixes {
        if !ALLOWED_PREFIXES.contains(prefix) {
            return Err(ValidationError::new("Invalid prefix"));
        }
    }

    Ok(())
}

/// Validates the destination path for creating a request entry.
/// Requirements:
/// - Path must not be absolute
/// - First segment must be 'requests'
/// - Path must not contain invalid characters
/// - Path must have at least one component after 'requests'
/// - Last segment cannot end with forbidden extension, e.g. ".request"

// const RESERVED_EXTENSIONS: [&str; 1] = ["request"];
// fn validate_request_destination(destination: &Path) -> Result<(), ValidationError> {
//     if destination.is_absolute() {
//         return Err(ValidationError::new("Destination path cannot be absolute"));
//     }
//
//     if destination.as_os_str().is_empty() {
//         return Err(ValidationError::new("Destination path cannot be empty"));
//     }
//
//     // Check that the first segment is 'requests'
//     let mut components = destination.components();
//     let first = components.next();
//
//     match first {
//         Some(std::path::Component::Normal(name)) => {
//             if name != "requests" {
//                 return Err(ValidationError::new(
//                     "First path segment must be 'requests'",
//                 ));
//             }
//         }
//         _ => {
//             return Err(ValidationError::new(
//                 "First path segment must be 'requests'",
//             ));
//         }
//     }
//
//     // Ensure there's at least one more component after 'requests'
//     if components.next().is_none() {
//         return Err(ValidationError::new(
//             "Path must contain at least one component after 'requests'",
//         ));
//     }
//
//     // Check for invalid path characters
//     let path_str = destination.to_string_lossy();
//     if path_str.contains("..") || path_str.contains("//") {
//         return Err(ValidationError::new("Path contains invalid sequences"));
//     }
//
//     let extension = destination
//         .extension()
//         .unwrap_or_default()
//         .to_string_lossy()
//         .to_string();
//     // Check for forbidden extensions
//     if RESERVED_EXTENSIONS.contains(&extension.as_ref()) {
//         return Err(ValidationError::new(
//             "Filename contains forbidden extension",
//         ));
//     }
//
//     Ok(())
// }

// Expand Entry

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ExpandEntryInput {
    pub id: Uuid,
    pub path: PathBuf,

    #[ts(optional)]
    pub depth: Option<u8>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ExpandEntryOutput {
    pub changes: WorktreeDiff,
}
