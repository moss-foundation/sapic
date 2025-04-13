use moss_environment::{
    environment::VariableCache,
    models::types::{VariableName, VariableValue},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CollectionEntity {
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VariableState {
    pub disabled: bool,
    pub order: Option<usize>,
    pub local_value: VariableValue,
}

impl From<VariableState> for VariableCache {
    fn from(value: VariableState) -> Self {
        Self {
            disabled: value.disabled,
            local_value: value.local_value,
            order: value.order,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EnvironmentEntity {
    pub order: Option<usize>,
    pub local_values: HashMap<VariableName, VariableState>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EditorGridPartStateEntity {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EditorPanelsPartStateEntity {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SidebarPartStateEntity {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PanelPartStateEntity {}
