mod editor;

pub use editor::*;

use moss_common::models::primitives::Identifier;
use moss_storage::workspace_storage::entities::state_store_entities::{
    ActivitybarPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::constants;

pub type EnvironmentName = String;

#[derive(Debug, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "types.ts")]
pub enum WorkspaceMode {
    #[serde(rename = "DESIGN_FIRST")]
    DesignFirst,

    #[serde(rename = "REQUEST_FIRST")]
    RequestFirst,
}

impl Default for WorkspaceMode {
    fn default() -> Self {
        Self::RequestFirst
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct CollectionInfo {
    #[ts(type = "Identifier")]
    pub id: Identifier,

    pub display_name: String,

    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfo {
    #[ts(type = "Identifier")]
    pub id: Identifier,
    #[ts(optional)]
    pub collection_id: Option<Identifier>,
    pub name: String,
    #[ts(optional)]
    pub order: Option<usize>,
}

// ------------------------------------------------------------
// Activitybar Part State
// ------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ActivitybarPartState {
    pub tree_view_group_id: String, // TODO: validate that this is an expected id value
}

impl From<ActivitybarPartStateEntity> for ActivitybarPartState {
    fn from(value: ActivitybarPartStateEntity) -> Self {
        ActivitybarPartState {
            tree_view_group_id: value
                .tree_view_group_id
                .unwrap_or(constants::TREE_VIEW_GROUP_COLLECTIONS.to_string()),
        }
    }
}

impl From<ActivitybarPartState> for ActivitybarPartStateEntity {
    fn from(value: ActivitybarPartState) -> Self {
        ActivitybarPartStateEntity {
            tree_view_group_id: Some(value.tree_view_group_id),
        }
    }
}

// ------------------------------------------------------------
// Sidebar Part State
// ------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct SidebarPartState {
    pub preferred_size: usize,
    pub is_visible: bool,
}

impl From<SidebarPartStateEntity> for SidebarPartState {
    fn from(value: SidebarPartStateEntity) -> Self {
        SidebarPartState {
            preferred_size: value.preferred_size,
            is_visible: value.is_visible,
        }
    }
}

impl From<SidebarPartState> for SidebarPartStateEntity {
    fn from(value: SidebarPartState) -> Self {
        SidebarPartStateEntity {
            preferred_size: value.preferred_size,
            is_visible: value.is_visible,
        }
    }
}

// ------------------------------------------------------------
// Panel Part State
// ------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PanelPartState {
    pub preferred_size: usize,
    pub is_visible: bool,
}

impl From<PanelPartStateEntity> for PanelPartState {
    fn from(value: PanelPartStateEntity) -> Self {
        PanelPartState {
            preferred_size: value.preferred_size,
            is_visible: value.is_visible,
        }
    }
}

impl From<PanelPartState> for PanelPartStateEntity {
    fn from(value: PanelPartState) -> Self {
        PanelPartStateEntity {
            preferred_size: value.preferred_size,
            is_visible: value.is_visible,
        }
    }
}
