mod editor;

use std::collections::HashMap;

pub use editor::*;

use moss_common::models::primitives::Identifier;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::primitives::{ActivitybarPosition, SidebarPosition};

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
pub struct ActivitybarItemStateInfo {
    pub id: String,
    pub order: usize,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ActivitybarPartStateInfo {
    pub last_active_container_id: Option<String>,
    pub position: ActivitybarPosition,
    pub items: Vec<ActivitybarItemStateInfo>,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "types.ts")]
// pub struct ActivitybarItemInfo {
//     pub id: String,
//     pub order: usize,
// }

// impl Merge<ActivitybarItemStateEntity> for ActivitybarItemInfo {
//     fn merge(&mut self, entity: ActivitybarItemStateEntity) -> &mut Self {
//         if let Some(order) = entity.order {
//             self.order = order;
//         }

//         self
//     }
// }

// impl From<ActivitybarPositionState> for ActivitybarPosition {
//     fn from(value: ActivitybarPositionState) -> Self {
//         match value {
//             ActivitybarPositionState::Default => ActivitybarPosition::Default,
//             ActivitybarPositionState::Top => ActivitybarPosition::Top,
//             ActivitybarPositionState::Bottom => ActivitybarPosition::Bottom,
//             ActivitybarPositionState::Hidden => ActivitybarPosition::Hidden,
//         }
//     }
// }

// impl From<ActivitybarPosition> for ActivitybarPositionState {
//     fn from(value: ActivitybarPosition) -> Self {
//         match value {
//             ActivitybarPosition::Default => ActivitybarPositionState::Default,
//             ActivitybarPosition::Top => ActivitybarPositionState::Top,
//             ActivitybarPosition::Bottom => ActivitybarPositionState::Bottom,
//             ActivitybarPosition::Hidden => ActivitybarPositionState::Hidden,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "types.ts")]
// pub struct ActivitybarPartState {
//     pub active_tree_view_group_id: String, // TODO: validate that this is an expected id value
//     pub position: ActivitybarPosition,
//     pub items: Vec<ActivitybarItemInfo>,
// }

// impl Default for ActivitybarPartState {
//     fn default() -> Self {
//         Self {
//             active_tree_view_group_id: constants::TREE_VIEW_GROUP_COLLECTIONS.to_string(),
//             position: ActivitybarPosition::Default,
//             items: vec![
//                 ActivitybarItemInfo {
//                     id: constants::TREE_VIEW_GROUP_COLLECTIONS.to_string(),
//                     order: 0,
//                 },
//                 ActivitybarItemInfo {
//                     id: constants::TREE_VIEW_GROUP_ENVIRONMENTS.to_string(),
//                     order: 1,
//                 },
//                 ActivitybarItemInfo {
//                     id: constants::TREE_VIEW_GROUP_MOCK_SERVERS.to_string(),
//                     order: 2,
//                 },
//             ],
//         }
//     }
// }

// impl From<ActivitybarPartStateEntity> for ActivitybarPartState {
//     fn from(value: ActivitybarPartStateEntity) -> Self {
//         ActivitybarPartState {
//             active_tree_view_group_id: value
//                 .active_tree_view_group_id
//                 .unwrap_or(constants::TREE_VIEW_GROUP_COLLECTIONS.to_string()),
//             position: value.position.unwrap_or_default(),

//         }
//     }
// }

// impl Merge<ActivitybarPartStateEntity> for ActivitybarPartState {
//     fn merge(&mut self, other: ActivitybarPartStateEntity) -> &mut Self {
//         if let Some(active_tree_view_group_id) = other.last_active_tree_view_item {
//             self.active_tree_view_group_id = active_tree_view_group_id;
//         }

//         if let Some(position) = other.position {
//             self.position = position.into();
//         }

//         if other.items.is_empty() {
//             return self;
//         }

//         for (id, item_entity) in other.items {
//             if let Some(index) = self.items.iter().position(|item| item.id == id) {
//                 self.items[index].merge(item_entity);
//             }
//         }

//         self
//     }
// }

// pub struct ActivitybarPart {
//     pub default: ActivitybarPartState,
//     pub current: ActivitybarPartState,
// }

// ------------------------------------------------------------
// Sidebar Part State
// ------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct SidebarPartStateInfo {
    pub position: SidebarPosition,
    pub preferred_size: usize, // TODO: rename to `size` ?
    pub visible: bool,
}

// impl From<SidebarPartStateEntity> for SidebarPartState {
//     fn from(value: SidebarPartStateEntity) -> Self {
//         SidebarPartState {
//             preferred_size: value.preferred_size,
//             is_visible: value.is_visible,
//         }
//     }
// }

// impl From<SidebarPartState> for SidebarPartStateEntity {
//     fn from(value: SidebarPartState) -> Self {
//         SidebarPartStateEntity {
//             preferred_size: value.preferred_size,
//             is_visible: value.is_visible,
//         }
//     }
// }

// ------------------------------------------------------------
// Panel Part State
// ------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PanelPartStateInfo {
    pub preferred_size: usize,
    pub visible: bool,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "types.ts")]
// pub struct PanelPartState {
//     pub preferred_size: usize,
//     pub is_visible: bool,
// }

// impl From<PanelPartStateEntity> for PanelPartState {
//     fn from(value: PanelPartStateEntity) -> Self {
//         PanelPartState {
//             preferred_size: value.preferred_size,
//             is_visible: value.is_visible,
//         }
//     }
// }

// impl From<PanelPartState> for PanelPartStateEntity {
//     fn from(value: PanelPartState) -> Self {
//         PanelPartStateEntity {
//             preferred_size: value.preferred_size,
//             is_visible: value.is_visible,
//         }
//     }
// }

// ------------------------------------------------------------
// Editor Part State
// ------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EditorPartStateInfo {
    pub grid: EditorGridState,
    #[ts(type = "Record<string, EditorPanelState>")]
    pub panels: HashMap<String, EditorPanelState>,
    #[ts(optional)]
    pub active_group: Option<String>,
}
