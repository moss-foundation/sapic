pub mod primitives;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;

use crate::environment::types::primitives::{VariableId, VariableName};

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "environment/types.ts")]
pub struct VariableInfo {
    pub id: VariableId,
    pub name: VariableName,
    #[ts(optional, type = "JsonValue")]
    pub global_value: Option<JsonValue>,
    #[ts(optional, type = "JsonValue")]
    pub local_value: Option<JsonValue>,
    pub disabled: bool,
    // pub kind: VariableKind,
    pub order: Option<isize>,
    pub desc: Option<String>,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "environment/types.ts")]
pub struct EnvironmentInfo {
    pub id: String,
    pub project_id: Option<String>,
    pub name: String,
    pub display_name: String,
    pub order: isize,
    pub color: Option<String>,
    pub variables: Vec<VariableInfo>,
}
