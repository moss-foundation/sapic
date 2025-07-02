use std::collections::HashMap;

use super::models::types::{VariableKind, VariableName, VariableValue};
use crate::constants::ID_LENGTH;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct VariableUpdate {
    pub kind: Option<VariableKind>,
    pub value: Option<VariableValue>,
    pub desc: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    pub kind: Option<VariableKind>,
    pub value: Option<VariableValue>,
    pub desc: Option<String>,
}

impl Variable {
    pub fn update(&mut self, update: VariableUpdate) {
        if let Some(kind) = update.kind {
            self.kind = Some(kind);
        }
        if let Some(value) = update.value {
            self.value = Some(value);
        }
        if let Some(desc) = update.desc {
            self.desc = Some(desc);
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileModel {
    // FIXME: Can the id be missing here?
    pub id: String,
    pub values: HashMap<VariableName, Variable>,
}

impl FileModel {
    // FIXME: Should we be able to create an environment filemodel without id?
    pub fn new() -> Self {
        Self {
            id: nanoid::nanoid!(ID_LENGTH),
            values: HashMap::new(),
        }
    }
}
