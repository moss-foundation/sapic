use sapic_base::environment::types::primitives::{EnvironmentId, VariableId};

pub const KEY_ENVIRONMENT_PREFIX: &'static str = "environment";
pub const KEY_VARIABLE_PREFIX: &'static str = "var";

// environment.{env_id}
pub fn key_environment(id: &EnvironmentId) -> String {
    format!("{KEY_ENVIRONMENT_PREFIX}.{id}")
}

// By prefixing the environment before variable id, we can easily fetch and remove all variables
// environment.{env_id}.var.{var_id}
pub fn key_variable(environment_id: &EnvironmentId, variable_id: &VariableId) -> String {
    format!(
        "{}.{KEY_VARIABLE_PREFIX}.{variable_id}",
        key_environment(environment_id)
    )
}

// environment.{env_id}.var.{var_id}.localValue
pub fn key_variable_local_value(
    environment_id: &EnvironmentId,
    variable_id: &VariableId,
) -> String {
    format!("{}.localValue", key_variable(environment_id, variable_id))
}
