use moss_storage2::models::primitives::StorageScope;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "StorageScope")]
#[ts(export, export_to = "primitives.ts")]
pub enum StorageScopeForFrontend {
    /// The stored data will be scoped to all workspaces across all profiles.
    Application,

    /// The stored data will be scoped to a specific workspace.
    Workspace(String),
}

impl From<StorageScopeForFrontend> for StorageScope {
    fn from(scope: StorageScopeForFrontend) -> Self {
        match scope {
            StorageScopeForFrontend::Application => StorageScope::Application,
            StorageScopeForFrontend::Workspace(workspace) => {
                StorageScope::Workspace(workspace.into())
            }
        }
    }
}

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub enum OptionalValue {
    None,
    // Using `Some` would make the frontend code look weird
    Value(#[ts(type = "JsonValue")] JsonValue),
}

impl From<Option<JsonValue>> for OptionalValue {
    fn from(value: Option<JsonValue>) -> Self {
        match value {
            Some(json_value) => OptionalValue::Value(json_value),
            None => OptionalValue::None,
        }
    }
}
