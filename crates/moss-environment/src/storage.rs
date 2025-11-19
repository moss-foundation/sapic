use crate::models::primitives::VariableId;

pub const KEY_VARIABLE_PREFIX: &'static str = "var";

pub fn key_variable(variable_id: &VariableId) -> String {
    format!("{KEY_VARIABLE_PREFIX}.{variable_id}")
}
pub fn key_variable_local_value(variable_id: &VariableId) -> String {
    format!("{KEY_VARIABLE_PREFIX}.{variable_id}.localValue")
}

pub fn key_variable_order(variable_id: &VariableId) -> String {
    format!("{KEY_VARIABLE_PREFIX}.{variable_id}.order")
}
