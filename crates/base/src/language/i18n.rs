/// Macro for creating a `LocalizedString` with a given key and origin.
///
/// # Example
///
/// ```rust
/// use base::localize;
///
/// // Create a LocalizedString with key and origin
/// let greeting = localize!("greeting.hello", "Hello, World!");
/// ```
#[macro_export]
macro_rules! localize {
    // Pattern for two arguments: key and origin
    ($key:expr, $origin:expr) => {
        $crate::language::types::LocalizedString {
            key: $key.to_string(),
            origin: $origin.to_string(),
        }
    };
}
