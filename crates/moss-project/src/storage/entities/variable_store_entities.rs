use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VariableEntity {
    pub disabled: bool,
    pub order: Option<isize>,
    pub local_value: JsonValue,
}
