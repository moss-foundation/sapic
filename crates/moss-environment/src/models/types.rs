use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;

pub type VariableName = String;
pub type EnvironmentName = String;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
pub struct VariableOptions {
    pub disabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
pub struct AddVariableParams {
    pub name: VariableName,
    pub global_value: JsonValue,
    pub local_value: JsonValue,
    // pub kind: Option<VariableKind>,
    pub order: isize,
    pub desc: Option<String>,
    pub options: VariableOptions,
}

pub struct UpdateVariableParams {}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "types.ts")]
pub enum VariableKind {
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "default")]
    Default,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct VariableInfo {
    pub name: VariableName,
    pub global_value: Option<JsonValue>,
    pub local_value: Option<JsonValue>,
    pub disabled: bool,
    pub kind: VariableKind,
    pub order: Option<isize>,
    pub desc: Option<String>,
}
