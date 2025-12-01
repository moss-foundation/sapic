pub mod primitives;

use std::ops::Deref;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::language::types::primitives::{LanguageCode, LanguageDirection};

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "language/types.ts")]
pub struct LanguageInfo {
    pub display_name: String,
    pub code: LanguageCode,
    pub direction: Option<LanguageDirection>,
}

/// A structure representing a localized string with a key and original text.
///
/// The `key` field serves as a unique identifier for the localization entry, while `origin`
/// is the fallback or original text.
///
/// # Example
///
/// ```rust
/// use base::localize;
///
/// // Using the `localize!` macro for concise creation
/// let welcome = localize!("welcome.message", "Welcome!");
/// ```
#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "language/types.ts")]
pub struct LocalizedString {
    /// The unique key identifying the localized string.
    pub key: String,

    /// The fallback string associated with the key, used when the key is not found.
    pub fallback: String,
}

impl std::fmt::Display for LocalizedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fallback)
    }
}

impl Deref for LocalizedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.fallback
    }
}

impl AsRef<str> for LocalizedString {
    fn as_ref(&self) -> &str {
        &self.fallback
    }
}
