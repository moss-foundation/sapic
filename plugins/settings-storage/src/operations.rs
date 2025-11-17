use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;
use validator::Validate;

use crate::types::primitives::*;

//
// Get Value
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetValueInput {
    pub scope: SettingScopeForFrontend,
    pub key: String,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetValueOutput {
    pub scope: SettingScopeForFrontend,
    pub key: String,
    #[ts(type = "JsonValue")]
    pub value: Option<JsonValue>,
}

//
// Update Value
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateValueInput {
    pub scope: SettingScopeForFrontend,
    pub key: String,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateValueOutput {}

//
// Remove Value
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RemoveValueInput {
    pub scope: SettingScopeForFrontend,
    pub key: String,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RemoveValueOutput {
    pub scope: SettingScopeForFrontend,
    pub key: String,
    #[ts(type = "JsonValue")]
    pub value: Option<JsonValue>,
}

//
// Batch Update Value
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateValueInput {
    pub scope: SettingScopeForFrontend,
    #[ts(type = "{ [key: string]: JsonValue }")]
    pub values: HashMap<String, JsonValue>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateValueOutput {}

//
// Batch Get Value
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchGetValueInput {
    pub scope: SettingScopeForFrontend,
    pub keys: Vec<String>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchGetValueOutput {
    pub scope: SettingScopeForFrontend,
    #[ts(type = "{ [key: string]: JsonValue | null }")]
    pub values: HashMap<String, Option<JsonValue>>,
}

//
// Batch Remove Value
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchRemoveValueInput {
    pub scope: SettingScopeForFrontend,
    pub keys: Vec<String>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchRemoveValueOutput {
    pub scope: SettingScopeForFrontend,
    #[ts(type = "{ [key: string]: JsonValue | null }")]
    pub values: HashMap<String, Option<JsonValue>>,
}
