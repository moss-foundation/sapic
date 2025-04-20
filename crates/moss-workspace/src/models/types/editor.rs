use moss_storage::workspace_storage::entities::state_store_entities::{
    EditorGridLeafDataEntity, EditorGridNodeEntity, EditorGridOrientationEntity,
    EditorGridStateEntity, EditorPanelStateEntity, EditorPartStateEntity, PanelRendererEntity,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "types.ts")]
pub enum EditorGridOrientation {
    Horizontal,
    Vertical,
}

impl From<EditorGridOrientation> for EditorGridOrientationEntity {
    fn from(value: EditorGridOrientation) -> Self {
        match value {
            EditorGridOrientation::Horizontal => EditorGridOrientationEntity::Horizontal,
            EditorGridOrientation::Vertical => EditorGridOrientationEntity::Vertical,
        }
    }
}

impl From<EditorGridOrientationEntity> for EditorGridOrientation {
    fn from(value: EditorGridOrientationEntity) -> Self {
        match value {
            EditorGridOrientationEntity::Horizontal => EditorGridOrientation::Horizontal,
            EditorGridOrientationEntity::Vertical => EditorGridOrientation::Vertical,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct EditorGridLeafData {
    pub views: Vec<String>,
    pub active_view: String,
    pub id: String,
}

impl From<EditorGridLeafData> for EditorGridLeafDataEntity {
    fn from(value: EditorGridLeafData) -> Self {
        EditorGridLeafDataEntity {
            views: value.views,
            active_view: value.active_view,
            id: value.id,
        }
    }
}

impl From<EditorGridLeafDataEntity> for EditorGridLeafData {
    fn from(value: EditorGridLeafDataEntity) -> Self {
        EditorGridLeafData {
            views: value.views,
            active_view: value.active_view,
            id: value.id,
        }
    }
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

impl From<EditorGridNode> for EditorGridNodeEntity {
    fn from(value: EditorGridNode) -> Self {
        match value {
            EditorGridNode::Branch { data, size } => EditorGridNodeEntity::Branch {
                data: data.into_iter().map(|node| node.into()).collect(),
                size,
            },
            EditorGridNode::Leaf { data, size } => EditorGridNodeEntity::Leaf {
                data: data.into(),
                size,
            },
        }
    }
}

impl From<EditorGridNodeEntity> for EditorGridNode {
    fn from(value: EditorGridNodeEntity) -> Self {
        match value {
            EditorGridNodeEntity::Branch { data, size } => EditorGridNode::Branch {
                data: data.into_iter().map(|node| node.into()).collect(),
                size,
            },
            EditorGridNodeEntity::Leaf { data, size } => EditorGridNode::Leaf {
                data: data.into(),
                size,
            },
        }
    }
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

impl From<EditorGridState> for EditorGridStateEntity {
    fn from(value: EditorGridState) -> Self {
        EditorGridStateEntity {
            root: value.root.into(),
            width: value.width,
            height: value.height,
            orientation: value.orientation.into(),
        }
    }
}

impl From<EditorGridStateEntity> for EditorGridState {
    fn from(value: EditorGridStateEntity) -> Self {
        EditorGridState {
            root: value.root.into(),
            width: value.width,
            height: value.height,
            orientation: value.orientation.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum PanelRenderer {
    OnlyWhenVisible,
    Always,
}

impl From<PanelRenderer> for PanelRendererEntity {
    fn from(value: PanelRenderer) -> Self {
        match value {
            PanelRenderer::OnlyWhenVisible => PanelRendererEntity::OnlyWhenVisible,
            PanelRenderer::Always => PanelRendererEntity::Always,
        }
    }
}

impl From<PanelRendererEntity> for PanelRenderer {
    fn from(value: PanelRendererEntity) -> Self {
        match value {
            PanelRendererEntity::OnlyWhenVisible => PanelRenderer::OnlyWhenVisible,
            PanelRendererEntity::Always => PanelRenderer::Always,
        }
    }
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

impl From<EditorPanelState> for EditorPanelStateEntity {
    fn from(value: EditorPanelState) -> Self {
        EditorPanelStateEntity {
            id: value.id,
            content_component: value.content_component,
            tab_component: value.tab_component,
            title: value.title,
            renderer: value.renderer.map(|renderer| renderer.into()),
            params: value.params,
            minimum_width: value.minimum_width,
            minimum_height: value.minimum_height,
            maximum_width: value.maximum_width,
            maximum_height: value.maximum_height,
        }
    }
}

impl From<EditorPanelStateEntity> for EditorPanelState {
    fn from(value: EditorPanelStateEntity) -> Self {
        EditorPanelState {
            id: value.id,
            content_component: value.content_component,
            tab_component: value.tab_component,
            title: value.title,
            renderer: value.renderer.map(|renderer| renderer.into()),
            params: value.params,
            minimum_width: value.minimum_width,
            minimum_height: value.minimum_height,
            maximum_width: value.maximum_width,
            maximum_height: value.maximum_height,
        }
    }
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

impl From<EditorPartState> for EditorPartStateEntity {
    fn from(value: EditorPartState) -> Self {
        EditorPartStateEntity {
            grid: value.grid.into(),
            panels: value
                .panels
                .into_iter()
                .map(|(key, panel)| (key, panel.into()))
                .collect(),
            active_group: value.active_group,
        }
    }
}

impl From<EditorPartStateEntity> for EditorPartState {
    fn from(value: EditorPartStateEntity) -> Self {
        EditorPartState {
            grid: value.grid.into(),
            panels: value
                .panels
                .into_iter()
                .map(|(key, panel)| (key, panel.into()))
                .collect(),
            active_group: value.active_group,
        }
    }
}
