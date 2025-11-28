use joinerror::error::ErrorMarker;

pub trait TauriResultExt<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T>;
    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T>;
    fn join_err_bare(self) -> joinerror::Result<T>;
}

impl<T> TauriResultExt<T> for Result<T, tauri::Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()).join_with::<E>(details))
    }

    fn join_err_bare(self) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use joinerror::Error;
//     use std::collections::HashMap;

//     fn create_validation_errors() -> joinerror::Result<()> {
//         let mut errors = ValidationErrors::new();
//         errors.add(
//             "name",
//             validator::ValidationError {
//                 code: "length".into(),
//                 message: Some("Name must be between 3 and 20 characters".into()),
//                 params: HashMap::new(),
//             },
//         );
//         errors.add(
//             "email",
//             validator::ValidationError {
//                 code: "email".into(),
//                 message: Some("Invalid email format".into()),
//                 params: HashMap::new(),
//             },
//         );
//         errors.add(
//             "age",
//             validator::ValidationError {
//                 code: "range".into(),
//                 message: Some("Age must be between 18 and 120".into()),
//                 params: HashMap::new(),
//             },
//         );

//         Err(Error::new::<InvalidInput>(errors.to_string()))
//     }

//     fn create_single_validation_error() -> joinerror::Result<()> {
//         let mut errors = ValidationErrors::new();

//         errors.add(
//             "name",
//             validator::ValidationError {
//                 code: "length".into(),
//                 message: Some("Name must be between 3 and 20 characters".into()),
//                 params: HashMap::new(),
//             },
//         );

//         Err(Error::new::<InvalidInput>(errors.to_string()))
//     }

//     #[test]
//     fn test_validation_error_from_multiple_errors() {
//         let err = create_validation_errors().unwrap_err();

//         // Check that the error has the correct type
//         assert!(err.is::<InvalidInput>());

//         // Check that the error message contains validation information
//         let error_string = err.to_string();
//         assert!(error_string.contains("invalid_input:"));
//         assert!(
//             error_string.contains("Name must be between 3 and 20 characters")
//                 || error_string.contains("Invalid email format")
//                 || error_string.contains("Age must be between 18 and 120")
//         );
//     }

//     #[test]
//     fn test_validation_error_from_single_error() {
//         let err = create_single_validation_error().unwrap_err();

//         assert!(err.is::<InvalidInput>());
//         let error_string = err.to_string();
//         assert!(error_string.contains("invalid_input:"));
//         assert!(error_string.contains("Name must be between 3 and 20 characters"));
//     }
// }
