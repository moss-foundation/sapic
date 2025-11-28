use std::{
    any::TypeId,
    fmt::{self, Display},
};

use serde::Serialize;

pub trait ErrorMarker: 'static {
    const MESSAGE: &'static str;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: Option<&'static str>,
    details: String,
    source: Option<Box<Error>>,
    _type: TypeId,
}

impl Error {
    pub fn new<E: ErrorMarker>(details: impl Into<String>) -> Self {
        let type_id = TypeId::of::<E>();
        Self {
            message: if type_id != TypeId::of::<()>() {
                Some(E::MESSAGE)
            } else {
                None
            },
            details: details.into(),
            source: None,
            _type: type_id,
        }
    }

    pub fn join<E: ErrorMarker>(self, details: impl Into<String>) -> Self {
        let type_id = TypeId::of::<E>();
        Self {
            message: if type_id != TypeId::of::<()>() {
                Some(E::MESSAGE)
            } else {
                None
            },
            details: details.into(),
            source: Some(Box::new(self)),
            _type: type_id,
        }
    }

    pub fn join_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Self {
        let type_id = TypeId::of::<E>();

        Self {
            message: if type_id != TypeId::of::<()>() {
                Some(E::MESSAGE)
            } else {
                None
            },
            details: details(),
            source: Some(Box::new(self)),
            _type: type_id,
        }
    }

    pub fn is<E: ErrorMarker>(&self) -> bool {
        if self._type == TypeId::of::<E>() {
            return true;
        }

        let mut current = &self.source;
        while let Some(source) = current {
            if source._type == TypeId::of::<E>() {
                return true;
            }
            current = &source.source;
        }

        false
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "{}: {}", msg, self.details)?,
            None => write!(f, "{}", self.details)?,
        }
        let mut current = &self.source;
        while let Some(source) = current {
            match &source.message {
                Some(msg) => write!(f, ": {}: {}", msg, source.details)?,
                None => write!(f, ": {}", source.details)?,
            }
            current = &source.source;
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;

    use crate::{Result, ResultExt};

    // Define test error types
    pub struct TestErrorInvalidInput;
    pub struct TestErrorNotFound;
    pub struct TestErrorDatabase;

    impl ErrorMarker for TestErrorInvalidInput {
        const MESSAGE: &'static str = "invalid_input";
    }

    impl ErrorMarker for TestErrorNotFound {
        const MESSAGE: &'static str = "not_found";
    }

    impl ErrorMarker for TestErrorDatabase {
        const MESSAGE: &'static str = "database_error";
    }

    // Test basic error creation
    #[test]
    fn test_error_creation() {
        let error = Error::new::<TestErrorInvalidInput>("Invalid user input");
        assert_eq!(error.message, Some("invalid_input"));
        assert_eq!(error.details, "Invalid user input");
        assert_eq!(error._type, TypeId::of::<TestErrorInvalidInput>());
        assert!(error.source.is_none());
        assert_eq!(error.to_string(), "invalid_input: Invalid user input");
    }

    #[test]
    fn test_error_creation_with_unit_type() {
        let error = Error::new::<()>("Generic error");
        assert_eq!(error.message, None);
        assert_eq!(error.details, "Generic error");
        assert_eq!(error._type, TypeId::of::<()>());
        assert!(error.source.is_none());
        assert_eq!(error.to_string(), "Generic error");
    }

    // Test error chaining
    #[test]
    fn test_error_chaining() {
        let error = Error::new::<TestErrorInvalidInput>("Invalid input")
            .join::<TestErrorNotFound>("User not found")
            .join::<TestErrorDatabase>("Database connection failed");
        assert_eq!(error.message, Some("database_error"));
        assert_eq!(error.details, "Database connection failed");
        assert_eq!(error._type, TypeId::of::<TestErrorDatabase>());
        assert!(error.is::<TestErrorDatabase>());
        assert!(error.is::<TestErrorNotFound>());
        assert!(error.is::<TestErrorInvalidInput>());
        assert_eq!(
            error.to_string(),
            "database_error: Database connection failed: not_found: User not found: invalid_input: Invalid input"
        );
    }

    // Test error type checking
    #[test]
    fn test_error_type_checking() {
        let error = Error::new::<TestErrorInvalidInput>("Invalid input")
            .join::<TestErrorNotFound>("User not found");
        assert!(error.is::<TestErrorNotFound>());
        assert!(error.is::<TestErrorInvalidInput>());
        assert!(!error.is::<TestErrorDatabase>());
        assert_eq!(
            error.to_string(),
            "not_found: User not found: invalid_input: Invalid input"
        );
    }

    // Test error with anyhow
    fn create_anyhow_error() -> anyhow::Result<()> {
        bail!("Something went wrong in anyhow")
    }

    fn create_anyhow_error_chain() -> anyhow::Result<()> {
        create_anyhow_error()?;
        Ok(())
    }

    #[test]
    fn test_anyhow_error_chaining() {
        let result: Result<()> = create_anyhow_error_chain()
            .map_err(|e| Error::new::<()>(e.to_string()))
            .join_err::<TestErrorInvalidInput>("Invalid input from anyhow")
            .join_err::<TestErrorNotFound>("User not found after anyhow error");
        let error = result.err().unwrap();
        assert_eq!(error.message, Some("not_found"));
        assert_eq!(error.details, "User not found after anyhow error");
        assert!(error.is::<TestErrorNotFound>());
        assert!(error.is::<TestErrorInvalidInput>());
        assert_eq!(
            error.to_string(),
            format!(
                "not_found: User not found after anyhow error: invalid_input: Invalid input from anyhow: {}",
                create_anyhow_error_chain().unwrap_err().to_string()
            )
        );
    }

    #[test]
    fn test_anyhow_error_with_multiple_steps() {
        let result: Result<()> = create_anyhow_error()
            .join_err::<TestErrorDatabase>("Database error")
            .join_err::<TestErrorInvalidInput>("Invalid input")
            .join_err::<TestErrorNotFound>("Final not found error");
        let error = result.err().unwrap();
        assert_eq!(error.message, Some("not_found"));
        assert_eq!(error.details, "Final not found error");
        assert!(error.is::<TestErrorNotFound>());
        assert!(error.is::<TestErrorInvalidInput>());
        assert!(error.is::<TestErrorDatabase>());
        assert_eq!(
            error.to_string(),
            format!(
                "not_found: Final not found error: invalid_input: Invalid input: database_error: Database error: {}",
                create_anyhow_error().unwrap_err().to_string()
            )
        );
    }

    // Test error cloning and equality
    #[test]
    fn test_error_cloning() {
        let error1 = Error::new::<TestErrorInvalidInput>("Test error")
            .join::<TestErrorNotFound>("Not found");
        let error2 = error1.clone();
        assert_eq!(error1, error2);
        assert!(error1.is::<TestErrorNotFound>());
        assert!(error2.is::<TestErrorNotFound>());
        assert_eq!(
            error1.to_string(),
            "not_found: Not found: invalid_input: Test error"
        );
        assert_eq!(error1.to_string(), error2.to_string());
    }

    // Test error with empty message
    #[test]
    fn test_error_with_empty_message() {
        let error = Error::new::<()>("Empty message error");
        assert_eq!(error.message, None);
        assert_eq!(error.details, "Empty message error");
        assert_eq!(error._type, TypeId::of::<()>());
        assert_eq!(error.to_string(), "Empty message error");
    }

    // Test complex anyhow error scenario
    fn simulate_database_operation() -> anyhow::Result<String> {
        // Simulate a database operation that fails
        bail!("Database connection timeout")
    }

    fn simulate_user_validation() -> anyhow::Result<()> {
        // Simulate user validation that fails
        bail!("User validation failed")
    }

    #[test]
    fn test_complex_anyhow_error_scenario() {
        let result: Result<()> = simulate_database_operation()
            .join_err::<TestErrorDatabase>("Database operation failed")
            .and_then(|_| {
                simulate_user_validation()
                    .join_err::<TestErrorInvalidInput>("User validation failed")
            })
            .join_err::<TestErrorNotFound>("User not found in final step");
        let error = result.err().unwrap();
        assert_eq!(error.message, Some("not_found"));
        assert_eq!(error.details, "User not found in final step");
        assert!(error.is::<TestErrorNotFound>());
        // Note: TestErrorInvalidInput is not in the chain because and_then creates a new error chain
        assert!(error.is::<TestErrorDatabase>());
        assert_eq!(
            error.to_string(),
            format!(
                "not_found: User not found in final step: database_error: Database operation failed: {}",
                simulate_database_operation().unwrap_err().to_string()
            )
        );
    }

    // Test error with different string types
    #[test]
    fn test_error_with_different_string_types() {
        let error1 = Error::new::<TestErrorInvalidInput>("String literal");
        let error2 = Error::new::<TestErrorInvalidInput>(String::from("String object"));
        let error3 = Error::new::<TestErrorInvalidInput>(format!("Formatted {}", "string"));
        assert_eq!(error1.details, "String literal");
        assert_eq!(error2.details, "String object");
        assert_eq!(error3.details, "Formatted string");
        assert_eq!(error1.to_string(), "invalid_input: String literal");
        assert_eq!(error2.to_string(), "invalid_input: String object");
        assert_eq!(error3.to_string(), "invalid_input: Formatted string");
    }

    // Test anyhow error with proper chaining
    #[test]
    fn test_anyhow_error_proper_chaining() {
        let result: Result<()> = create_anyhow_error()
            .join_err::<TestErrorDatabase>("Database error")
            .join_err::<TestErrorInvalidInput>("Invalid input")
            .join_err::<TestErrorNotFound>("Final error");
        let error = result.err().unwrap();
        assert_eq!(error.message, Some("not_found"));
        assert_eq!(error.details, "Final error");
        assert!(error.is::<TestErrorNotFound>());
        assert!(error.is::<TestErrorInvalidInput>());
        assert!(error.is::<TestErrorDatabase>());
        assert_eq!(
            error.to_string(),
            format!(
                "not_found: Final error: invalid_input: Invalid input: database_error: Database error: {}",
                create_anyhow_error().unwrap_err().to_string()
            )
        );
    }
}
