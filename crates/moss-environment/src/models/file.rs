use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::types::{VariableKind, VariableName, VariableValue};

#[derive(Debug)]
pub struct EnvironmentFileVariableUpdate {
    pub kind: Option<VariableKind>,
    pub value: Option<VariableValue>,
    pub desc: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentFileVariable {
    pub kind: VariableKind,
    pub value: VariableValue,
    pub desc: Option<String>,
}

impl EnvironmentFileVariable {
    pub fn update(&mut self, update: EnvironmentFileVariableUpdate) {
        if let Some(kind) = update.kind {
            self.kind = kind;
        }
        if let Some(value) = update.value {
            self.value = value;
        }
        if let Some(desc) = update.desc {
            self.desc = Some(desc);
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct EnvironmentFile {
    pub values: HashMap<VariableName, EnvironmentFileVariable>,
}
