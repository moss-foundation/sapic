pub mod defaults;
pub mod error;
pub mod ext;

pub use error::{Error, ErrorMarker};
pub use joinerror_macros::errors;

pub type Result<T> = std::result::Result<T, Error>;

pub trait ResultExt<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T>;

    fn join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T>;
}

pub trait OptionExt<T> {
    fn ok_or_join_err<E: ErrorMarker>(self, details: impl Into<String>) -> Result<T>;
    fn ok_or_join_err_with<E: ErrorMarker>(self, details: impl FnOnce() -> String) -> Result<T>;
}

/// Macro for early return with an error
///
/// This macro creates an Error and returns early from the current function.
/// It supports multiple forms:
/// - `bail!("error message")` - creates an error with unit type
/// - `bail!("format {}", arg)` - creates an error with formatted message
/// - `bail!(ErrorType, "error message")` - creates an error with the specified ErrorMarker type
/// - `bail!(ErrorType, "format {}", arg)` - creates an error with ErrorMarker type and formatted message
///
/// # Examples
///
/// ```
/// use joinerror::{bail, Result, ErrorMarker};
///
/// struct MyError;
/// impl ErrorMarker for MyError {
///     const MESSAGE: &'static str = "my_error";
/// }
///
/// fn example1() -> Result<()> {
///     bail!("Something went wrong");
/// }
///
/// fn example2() -> Result<()> {
///     bail!("Value {} is invalid", 42);
/// }
///
/// fn example3() -> Result<()> {
///     bail!(MyError, "Specific error occurred");
/// }
///
/// fn example4() -> Result<()> {
///     let value = 42;
///     bail!(MyError, "Error with value: {}", value);
/// }
/// ```
#[macro_export]
macro_rules! bail {
    ($msg:literal) => {
        return Err($crate::Error::new::<()>($msg))
    };
    ($fmt:literal, $($arg:tt)*) => {
        return Err($crate::Error::new::<()>(format!($fmt, $($arg)*)))
    };
    ($error_type:ty, $msg:literal) => {
        return Err($crate::Error::new::<$error_type>($msg))
    };
    ($error_type:ty, $fmt:literal, $($arg:tt)*) => {
        return Err($crate::Error::new::<$error_type>(format!($fmt, $($arg)*)))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestError;
    impl ErrorMarker for TestError {
        const MESSAGE: &'static str = "test_error";
    }

    #[test]
    fn test_bail_macro_basic() {
        fn test_function() -> Result<()> {
            bail!("Test error message");
        }

        let result = test_function();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Test error message");
    }

    #[test]
    fn test_bail_macro_with_error_type() {
        fn test_function() -> Result<()> {
            bail!(TestError, "Specific test error");
        }

        let result = test_function();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "test_error: Specific test error");
        assert!(error.is::<TestError>());
    }

    #[test]
    fn test_bail_macro_with_format() {
        fn test_function(value: i32) -> Result<()> {
            bail!("Value is too large: {}", value);
        }

        let result = test_function(42);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Value is too large: 42");
    }

    #[test]
    fn test_bail_macro_with_error_type_and_format() {
        fn test_function(value: &str) -> Result<()> {
            bail!(TestError, "Invalid value: {}", value);
        }

        let result = test_function("test");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "test_error: Invalid value: test");
        assert!(error.is::<TestError>());
    }
}
