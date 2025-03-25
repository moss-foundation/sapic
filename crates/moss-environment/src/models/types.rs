use serde::{Deserialize, Serialize};
use serde_json::Number;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "types.ts")]
pub enum VariableKind {
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "default")]
    Default,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(untagged)]
pub enum VariableValue {
    String(String),
    Number(Number),
    Boolean(bool),
}
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct VariableInfo {
    pub kind: VariableKind,
    pub value: VariableValue,
}

pub type VariableName = String;
