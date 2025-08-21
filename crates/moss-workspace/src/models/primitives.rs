use moss_id_macro::generate_id_type;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

generate_id_type!(CollectionId);

/// @category Type
#[derive(Debug, PartialEq, Serialize, Deserialize, TS, Clone)]
#[ts(export, export_to = "primitives.ts")]
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

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ActivitybarPosition {
    Default,
    Top,
    Bottom,
    Hidden,
}

impl Default for ActivitybarPosition {
    fn default() -> Self {
        Self::Default
    }
}

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum SidebarPosition {
    Left,
    Right,
}

/// @category Primitive
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum EditorGridOrientation {
    Horizontal,
    Vertical,
}

/// @category Primitive
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub enum PanelRenderer {
    OnlyWhenVisible,
    Always,
}
