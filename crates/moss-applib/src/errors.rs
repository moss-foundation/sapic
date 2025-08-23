//! Global application error types
//!
//! This module contains only universal error types that are relevant across
//! the entire application. Domain-specific errors or errors specific to
//! particular crates should be declared directly in those crates where
//! they are used, not in this global module.
//!
//! Implementing *ResultExt traits in this module is only acceptable for types
//! from external libraries. All other internal, application-level extensions
//! should be implemented in their own dedicated crates.

use joinerror::{error::ErrorMarker, errors};
use validator::ValidationErrors;

errors! {
    /// The operation was rejected because the system is not in a state required
    /// for the operation's execution. For example, the workspace to be described or
    /// deleted does not opened yet, etc.
    FailedPrecondition => "failed_precondition",

    /// The operation was rejected because the input was invalid.
    InvalidInput => "invalid_input",

    /// The entity that a client attempted to access (e.g., file or directory) does not exist.
    NotFound => "not_found",

    /// The entity that a client attempted to create (e.g., file or directory) already exists.
    AlreadyExists => "already_exists",

    /// The operation was rejected because the resource is not available.
    /// Note that it is not always safe to retry non-idempotent operations.
    Unavailable => "unavailable",

    /// This means that some invariants expected by the underlying system have been broken.
    /// This error is reserved for serious errors.
    Internal => "internal",

    /// This error is reserved for errors that are not covered by the other error codes.
    Unknown => "unknown",

    /// This error means that the operation did not finish within the specified deadline.
    Timeout => "timeout",
}

pub type ValidationResult<T> = Result<T, ValidationErrors>;
pub trait ValidationResultExt<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T>;

    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T>;

    fn join_err_bare(self) -> joinerror::Result<T>;
}

impl<T> ValidationResultExt<T> for ValidationResult<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<InvalidInput>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T> {
        self.map_err(|e| {
            joinerror::Error::new::<InvalidInput>(e.to_string()).join_with::<E>(details)
        })
    }

    fn join_err_bare(self) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<InvalidInput>(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use joinerror::Error;
    use std::collections::HashMap;

    fn create_validation_errors() -> joinerror::Result<()> {
        let mut errors = ValidationErrors::new();
        errors.add(
            "name",
            validator::ValidationError {
                code: "length".into(),
                message: Some("Name must be between 3 and 20 characters".into()),
                params: HashMap::new(),
            },
        );
        errors.add(
            "email",
            validator::ValidationError {
                code: "email".into(),
                message: Some("Invalid email format".into()),
                params: HashMap::new(),
            },
        );
        errors.add(
            "age",
            validator::ValidationError {
                code: "range".into(),
                message: Some("Age must be between 18 and 120".into()),
                params: HashMap::new(),
            },
        );

        Err(Error::new::<InvalidInput>(errors.to_string()))
    }

    fn create_single_validation_error() -> joinerror::Result<()> {
        let mut errors = ValidationErrors::new();

        errors.add(
            "name",
            validator::ValidationError {
                code: "length".into(),
                message: Some("Name must be between 3 and 20 characters".into()),
                params: HashMap::new(),
            },
        );

        Err(Error::new::<InvalidInput>(errors.to_string()))
    }

    #[test]
    fn test_validation_error_from_multiple_errors() {
        let err = create_validation_errors().unwrap_err();

        // Check that the error has the correct type
        assert!(err.is::<InvalidInput>());

        // Check that the error message contains validation information
        let error_string = err.to_string();
        assert!(error_string.contains("invalid_input:"));
        assert!(
            error_string.contains("Name must be between 3 and 20 characters")
                || error_string.contains("Invalid email format")
                || error_string.contains("Age must be between 18 and 120")
        );
    }

    #[test]
    fn test_validation_error_from_single_error() {
        let err = create_single_validation_error().unwrap_err();

        assert!(err.is::<InvalidInput>());
        let error_string = err.to_string();
        assert!(error_string.contains("invalid_input:"));
        assert!(error_string.contains("Name must be between 3 and 20 characters"));
    }
}
