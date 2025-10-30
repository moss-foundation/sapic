use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;
use validator::Validate;

use crate::models::primitives::Scope;

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetItemInput {
    pub key: String,
    pub scope: Scope,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetItemOutput {
    pub key: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
    pub scope: Scope,
}

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct PutItemInput {
    pub key: String,
    pub scope: Scope,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct PutItemOutput {
    pub success: bool,
}

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RemoveItemInput {
    pub key: String,
    pub scope: Scope,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RemoveItemOutput {
    pub success: bool,
}
