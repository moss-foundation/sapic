use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use ts_rs::TS;
use validator::Validate;

use crate::models::primitives::*;

//
// Get Item
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetItemInput {
    pub key: String,
    pub scope: StorageScopeForFrontend,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetItemOutput {
    pub key: String,
    pub value: OptionalValue,
    pub scope: StorageScopeForFrontend,
}

//
// Put Item
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct PutItemInput {
    pub key: String,
    pub scope: StorageScopeForFrontend,
    #[ts(type = "JsonValue")]
    pub value: JsonValue,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct PutItemOutput {}

//
// Remove Item
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RemoveItemInput {
    pub key: String,
    pub scope: StorageScopeForFrontend,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RemoveItemOutput {
    pub scope: StorageScopeForFrontend,
    #[ts(type = "JsonValue")]
    pub value: Option<JsonValue>,
}

//
// Batch Put Item
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchPutItemInput {
    pub scope: StorageScopeForFrontend,
    #[ts(type = "{ [key: string]: JsonValue }")]
    pub items: HashMap<String, JsonValue>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchPutItemOutput {}

//
// Batch Remove Item
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchRemoveItemInput {
    pub scope: StorageScopeForFrontend,
    pub keys: Vec<String>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchRemoveItemOutput {
    pub scope: StorageScopeForFrontend,
    #[ts(type = "{ [key: string]: JsonValue | null }")]
    pub items: HashMap<String, Option<JsonValue>>,
}

//
// Batch Get Item
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchGetItemInput {
    pub scope: StorageScopeForFrontend,
    pub keys: Vec<String>,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchGetItemOutput {
    pub scope: StorageScopeForFrontend,
    #[ts(type = "{ [key: string]: JsonValue | null }")]
    pub items: HashMap<String, Option<JsonValue>>,
}

//
// Batch Get Item By Prefix
//

/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchGetItemByPrefixInput {
    pub scope: StorageScopeForFrontend,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchGetItemByPrefixOutput {
    pub scope: StorageScopeForFrontend,
    #[ts(type = "{ [key: string]: JsonValue }")]
    pub items: HashMap<String, JsonValue>,
}

//
// Batch Remove Item By Prefix
//
/// @category Operation
#[derive(Debug, Clone, Deserialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchRemoveItemByPrefixInput {
    pub scope: StorageScopeForFrontend,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchRemoveItemByPrefixOutput {
    pub scope: StorageScopeForFrontend,
    #[ts(type = "{ [key: string]: JsonValue }")]
    pub items: HashMap<String, JsonValue>,
}
