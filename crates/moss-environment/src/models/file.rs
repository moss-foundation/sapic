use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::types::{VariableInfo, VariableName};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct EnvironmentFile {
    pub values: HashMap<VariableName, VariableInfo>,
}
