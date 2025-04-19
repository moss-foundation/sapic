use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "types.ts")]
pub enum EditorGridOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EditorGridLeafData {
    pub views: Vec<String>,
    pub active_view: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EditorGridNode {
    Branch {
        data: Vec<EditorGridNode>,
        size: f64,
    },
    Leaf {
        data: EditorGridLeafData,
        size: f64,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EditorGridState {
    pub root: EditorGridNode,
    pub width: f64,
    pub height: f64,
    pub orientation: EditorGridOrientation,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum PanelRenderer {
    OnlyWhenVisible,
    Always,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EditorPanelState {
    pub id: String,
    #[ts(optional)]
    pub content_component: Option<String>,
    #[ts(optional)]
    pub tab_component: Option<String>,
    #[ts(optional)]
    pub title: Option<String>,
    #[ts(optional)]
    pub renderer: Option<PanelRenderer>,
    #[ts(optional)]
    #[ts(type = "{ [key: string]: JsonValue }")]
    pub params: Option<HashMap<String, JsonValue>>,
    #[ts(optional)]
    pub minimum_width: Option<f64>,
    #[ts(optional)]
    pub minimum_height: Option<f64>,
    #[ts(optional)]
    pub maximum_width: Option<f64>,
    #[ts(optional)]
    pub maximum_height: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EditorPartState {
    pub grid: EditorGridState,
    pub panels: HashMap<String, EditorPanelState>,
    #[ts(optional)]
    pub active_group: Option<String>,
}
