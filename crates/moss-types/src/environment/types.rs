use serde::{Deserialize, Serialize};
use serde_json::Number;
use ts_rs::TS;

pub type VariableName = String;
pub type EnvironmentName = String;

#[derive(Clone, Debug, Deserialize, Serialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "environment/types.ts")]
pub enum VariableKind {
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "default")]
    Default,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, TS)]
#[ts(export, export_to = "environment/types.ts")]
#[serde(untagged)]
pub enum VariableValue {
    String(String),
    Number(Number),
    Boolean(bool),
    Null,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "environment/types.ts")]
pub struct VariableInfo {
    pub name: VariableName,
    pub global_value: VariableValue,
    pub local_value: VariableValue,
    pub disabled: bool,
    pub kind: VariableKind,
    pub order: Option<usize>,
    pub desc: Option<String>,
}
