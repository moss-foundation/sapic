pub mod primitives;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;

use super::types::primitives::*;

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ConfigurationSchema {
    pub id: String,
    pub parent_id: Option<String>,
    pub order: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<ParameterSchema>,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ParameterSchema {
    pub id: String,
    #[ts(optional, type = "JsonValue")]
    pub default: Option<JsonValue>,
    pub typ: ConfigurationParameterType,
    pub description: Option<String>,
    pub maximum: Option<u64>,
    pub minimum: Option<u64>,
    pub protected: bool,
    pub order: Option<i64>,
    pub tags: Vec<String>,
}
