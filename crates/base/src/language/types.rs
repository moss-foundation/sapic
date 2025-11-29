pub mod primitives;

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

    /// The original text or fallback string associated with the key.
    pub origin: String,
}
