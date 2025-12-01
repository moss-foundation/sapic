pub const NO_TRANSLATE_KEY: &str = "__NO_TRANSLATE__";

/// Macro for creating a `LocalizedString` with a given key and fallback.
///
/// # Example
///
/// ```rust
/// use base::localize;
///
/// // Create a LocalizedString with key and fallback
/// let greeting = localize!("greeting.hello", "Hello, World!");
/// ```
#[macro_export]
macro_rules! localize {
    // Pattern for two arguments: key and fallback
    ($key:expr, $fallback:expr) => {
        $crate::language::types::LocalizedString {
            key: $key.to_string(),
            fallback: $fallback.to_string(),
        }
    };
}
