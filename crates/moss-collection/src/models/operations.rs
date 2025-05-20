use serde::Serialize;
use serde_json::Value as JsonValue;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use super::{
    primitives::ChangesDiffSet,
    types::{Classification, PathChangeKind},
};
use crate::models::{
    primitives::EntryId,
    types::{HeaderParamItem, HttpMethod, PathParamItem, QueryParamItem, RequestBody},
};

/// All the path and file names passed in the input should be unencoded.
/// For example, a name of "workspace.name" will be encoded as "workspace%2Ename"
/// The frontend should simply use the name and path used in the user's original input
///

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
// TODO: Validate that destination matches with classification
// #[validate(schema(function = "validate_category", skip_on_field_errors = false))]
pub struct CreateEntryInput {
    // TODO: Validate against all possible classification
    #[validate(custom(function = "validate_request_destination"))]
    pub destination: PathBuf,
    pub classification: Classification,
    pub specification: Option<JsonValue>,
    // TODO: use RequestProtocol?
    pub protocol: Option<String>,
    pub order: Option<usize>,
    pub is_dir: bool,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateEntryOutput {
    pub physical_changes: ChangesDiffSet,
    pub virtual_changes: ChangesDiffSet,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryInput {
    pub id: EntryId,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryOutput {
    pub physical_changes: ChangesDiffSet,
    pub virtual_changes: ChangesDiffSet,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEntryInput {
    pub id: EntryId,
    pub name: Option<String>,
    pub classification: Option<Classification>,
    pub specification: Option<JsonValue>,
    // TODO: use RequestProtocol?
    pub protocol: Option<String>,
    pub order: Option<usize>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEntryOutput {
    pub physical_changes: ChangesDiffSet,
    pub virtual_changes: ChangesDiffSet,
}
// ------------Old API----------------

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

// Delete Request Entry
#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteRequestEntryInput {
    pub id: EntryId,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteRequestEntryOutput {
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

// Update Request Entry

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateRequestEntryInput {
    pub id: EntryId,

    /// A new name for the request, if provided,
    /// the request will be renamed to this name.
    #[ts(optional)]
    #[validate(length(min = 1))]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateRequestEntryOutput {
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
/// - Last segment cannot end with forbidden extension, e.g. ".request"

const RESERVED_EXTENSIONS: [&str; 1] = ["request"];
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

    let extension = destination
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    // Check for forbidden extensions
    if RESERVED_EXTENSIONS.contains(&extension.as_ref()) {
        return Err(ValidationError::new(
            "Filename contains forbidden extension",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_request_destination_correct() {
        let path = Path::new("requests").join("1");
        assert!(validate_request_destination(&path).is_ok());
    }

    // TODO: Test validate absolute path in a cross platform manner
    #[test]
    fn validate_request_destination_empty() {
        let path = PathBuf::new();
        assert!(matches!(
            validate_request_destination(&path),
            Err(ValidationError { .. })
        ));
    }

    #[test]
    fn validate_request_destination_no_components_after_requests() {
        let path = PathBuf::from("requests");
        assert!(matches!(
            validate_request_destination(&path),
            Err(ValidationError { .. })
        ));
    }

    #[test]
    fn validate_request_destination_first_component_not_requests() {
        let path = PathBuf::from("non-requests").join("1");
        assert!(matches!(
            validate_request_destination(&path),
            Err(ValidationError { .. })
        ));
    }

    #[test]
    fn validate_request_destination_invalid_path_characters() {
        let path = PathBuf::from("requests\\1\\..");
        assert!(matches!(
            validate_request_destination(&path),
            Err(ValidationError { .. })
        ));

        let path = PathBuf::from("requests\\1//");
        assert!(matches!(
            validate_request_destination(&path),
            Err(ValidationError { .. })
        ));
    }

    #[test]
    fn validate_request_destination_forbidden_extension() {
        let path = PathBuf::from("requests").join("1.request");
        assert!(matches!(
            validate_request_destination(&path),
            Err(ValidationError { .. })
        ))
    }
}
