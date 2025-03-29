use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::types::{VariableKind, VariableName, VariableValue};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentFileVariable {
    pub kind: VariableKind,
    pub value: VariableValue,
    pub desc: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct EnvironmentFile {
    pub values: HashMap<VariableName, EnvironmentFileVariable>,
}
