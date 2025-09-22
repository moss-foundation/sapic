pub mod models;

use moss_text::ReadOnlyStr;
use static_json::Value as JsonValueRef;

use crate::models::primitives::ParameterType;

#[derive(Debug)]
pub struct ParameterDecl {
    pub id: ReadOnlyStr,
    pub default: Option<JsonValueRef<'static>>,
    pub typ: ParameterType,
    pub description: Option<&'static str>,
    pub maximum: Option<u64>,
    pub minimum: Option<u64>,

    /// Excluded parameters are hidden from the UI but can still be registered.
    pub excluded: bool,

    /// Indicates if this setting is protected from addon overrides.
    pub protected: bool,

    /// The order in which the parameter appears within its group in the settings UI.
    pub order: Option<i64>,
    pub tags: &'static [ReadOnlyStr],
}

#[derive(Debug)]
pub struct ConfigurationDecl {
    pub id: Option<ReadOnlyStr>,
    pub parent_id: Option<ReadOnlyStr>,

    /// The order in which the parameter appears within its group in the settings UI.
    pub order: Option<i64>,
    pub name: Option<&'static str>,
    pub description: Option<&'static str>,
    pub parameters: &'static [ParameterDecl],
}

inventory::collect!(ConfigurationDecl);
