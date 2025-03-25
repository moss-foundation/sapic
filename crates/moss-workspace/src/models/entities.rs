use std::collections::HashMap;

use moss_environment::models::types::VariableName;
use moss_environment::models::types::VariableValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CollectionEntity {
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EnvironmentEntity {
    pub local_values: HashMap<VariableName, VariableValue>,
}
