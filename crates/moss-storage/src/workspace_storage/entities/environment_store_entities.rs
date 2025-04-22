use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VariableStateEntity {
    pub disabled: bool,
    pub order: Option<usize>,
    pub local_value: JsonValue,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EnvironmentEntity {
    pub order: Option<usize>,
    /// The key is the variable name, the value is the variable state.
    pub local_values: HashMap<String, VariableStateEntity>,
}
