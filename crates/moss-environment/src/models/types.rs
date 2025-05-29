use serde::{Deserialize, Serialize};
use serde_json::{Number, Value as JsonValue};
use ts_rs::TS;

pub type VariableName = String;
pub type EnvironmentName = String;

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
    Null,
}

impl TryFrom<JsonValue> for VariableValue {
    type Error = anyhow::Error;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::String(s) => Ok(VariableValue::String(s)),
            JsonValue::Number(n) => Ok(VariableValue::Number(n)),
            JsonValue::Bool(b) => Ok(VariableValue::Boolean(b)),
            JsonValue::Null => Ok(VariableValue::Null),
            _ => Err(anyhow::anyhow!(
                "Unsupported variable value type: {:?}",
                value
            )),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct VariableInfo {
    pub name: VariableName,
    pub global_value: VariableValue,
    pub local_value: VariableValue,
    pub disabled: bool,
    pub kind: VariableKind,
    pub order: Option<usize>,
    pub desc: Option<String>,
}
