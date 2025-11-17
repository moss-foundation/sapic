use moss_text::ReadOnlyStr;
use serde::Deserialize;
use serde_json::Value as JsonValue;

use super::types::primitives::ConfigurationParameterType as ParameterType;

#[derive(Debug, Deserialize)]
pub struct ParameterDecl {
    pub id: ReadOnlyStr,
    pub default: Option<JsonValue>,
    #[serde(rename = "type")]
    pub typ: ParameterType,
    pub description: Option<String>,
    pub maximum: Option<u64>,
    pub minimum: Option<u64>,
    pub excluded: bool,
    pub protected: bool,
    pub order: Option<i64>,
    #[serde(default)]
    pub tags: Vec<ReadOnlyStr>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigurationDecl {
    pub id: Option<ReadOnlyStr>,
    pub parent_id: Option<ReadOnlyStr>,
    pub order: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<ParameterDecl>,
}
