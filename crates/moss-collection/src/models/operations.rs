use serde::Serialize;
use serde_json::Value as JsonValue;
use std::path::{Path, PathBuf};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use super::{primitives::ChangesDiffSet, types::Classification};
use crate::models::primitives::EntryId;
use crate::models::specification::SpecificationContent;
use crate::models::types::RequestProtocol;

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
// TODO: Validate that destination matches with classification
// TODO: Validate that specification content type matches with entry type
// #[validate(schema(function = "validate_category", skip_on_field_errors = false))]
pub struct CreateEntryInput {
    // TODO: Validate against all possible classification
    #[validate(custom(function = "validate_request_destination"))]
    pub destination: PathBuf,
    pub classification: Classification,
    // FIXME: Figure out a strategy for passing and receiving specification
    #[ts(optional)]
    pub specification: Option<SpecificationContent>,
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
    #[serde(skip)]
    #[ts(skip)]
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
    #[serde(skip)]
    #[ts(skip)]
    pub physical_changes: ChangesDiffSet,
    pub virtual_changes: ChangesDiffSet,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEntryInput {
    pub id: EntryId,
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
    #[serde(skip)]
    #[ts(skip)]
    pub physical_changes: ChangesDiffSet,
    pub virtual_changes: ChangesDiffSet,
}

// #[derive(Clone, Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub enum CreateRequestProtocolSpecificPayload {
//     Http {
//         method: HttpMethod,
//         query_params: Vec<QueryParamItem>,
//         path_params: Vec<PathParamItem>,
//         headers: Vec<HeaderParamItem>,
//         #[ts(optional)]
//         body: Option<RequestBody>,
//     },
// }

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
