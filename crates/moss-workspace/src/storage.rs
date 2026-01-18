use sapic_base::{
    environment::types::primitives::EnvironmentId, project::types::primitives::ProjectId,
};

pub static KEY_EXPANDED_ITEMS: &'static str = "expandedItems";

pub static KEY_EXPANDED_ENVIRONMENT_GROUPS: &'static str = "expandedEnvironmentGroups";
pub static KEY_ACTIVE_ENVIRONMENT: &'static str = "activeEnvironment";

pub static KEY_PROJECT_PREFIX: &'static str = "project";
pub static KEY_ENVIRONMENT_GROUP_PREFIX: &'static str = "environmentGroup";
pub static KEY_ENVIRONMENT_PREFIX: &'static str = "environment";

pub fn key_project(id: &ProjectId) -> String {
    format!("{KEY_PROJECT_PREFIX}.{id}")
}

pub fn key_project_order(id: &ProjectId) -> String {
    format!("{KEY_PROJECT_PREFIX}.{id}.order")
}

pub fn key_environment_group(id: &ProjectId) -> String {
    format!("{KEY_ENVIRONMENT_GROUP_PREFIX}.{id}")
}

pub fn key_environment_group_order(id: &ProjectId) -> String {
    format!("{KEY_ENVIRONMENT_GROUP_PREFIX}.{id}.order")
}

pub fn key_environment(id: &EnvironmentId) -> String {
    format!("{KEY_ENVIRONMENT_PREFIX}.{id}")
}

pub fn key_environment_order(id: &EnvironmentId) -> String {
    format!("{KEY_ENVIRONMENT_PREFIX}.{id}.order")
}
