use moss_environment::{
    environment::VariableCache,
    models::types::{VariableName, VariableValue},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::{EditorGridState, EditorPanelState};

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
pub struct EditorPartStateEntity {
    pub grid: EditorGridState,
    pub panels: HashMap<String, EditorPanelState>,
    pub active_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SidebarPartStateEntity {
    pub preferred_size: usize,
    pub is_visible: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PanelPartStateEntity {
    pub preferred_size: usize,
    pub is_visible: bool,
}
