use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EditorGridOrientationEntity {
    Horizontal,
    Vertical,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EditorGridLeafDataEntity {
    pub views: Vec<String>,
    pub active_view: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum EditorGridNodeEntity {
    Branch {
        data: Vec<EditorGridNodeEntity>,
        size: f64,
    },
    Leaf {
        data: EditorGridLeafDataEntity,
        size: f64,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EditorGridStateEntity {
    pub root: EditorGridNodeEntity,
    pub width: f64,
    pub height: f64,
    pub orientation: EditorGridOrientationEntity,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PanelRendererEntity {
    OnlyWhenVisible,
    Always,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EditorPanelStateEntity {
    pub id: String,
    pub content_component: Option<String>,
    pub tab_component: Option<String>,
    pub title: Option<String>,
    pub renderer: Option<PanelRendererEntity>,
    pub params: Option<HashMap<String, JsonValue>>,
    pub minimum_width: Option<f64>,
    pub minimum_height: Option<f64>,
    pub maximum_width: Option<f64>,
    pub maximum_height: Option<f64>,
}
