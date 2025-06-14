mod editor;
pub use editor::*;

use moss_common::models::primitives::Identifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use super::primitives::{ActivitybarPosition, SidebarPosition};

pub type EnvironmentName = String;

#[derive(Debug, PartialEq, Serialize, Deserialize, TS, Clone)]
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

// ------------------------------------------------------------
// Sidebar Part State
// ------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct SidebarPartStateInfo {
    pub position: SidebarPosition,
    pub size: usize,
    pub visible: bool,
}

// ------------------------------------------------------------
// Panel Part State
// ------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PanelPartStateInfo {
    pub size: usize,
    pub visible: bool,
}

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
