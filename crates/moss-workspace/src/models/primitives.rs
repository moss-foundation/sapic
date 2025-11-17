use moss_id_macro::ids;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ids!([WorkspaceId]);

/// @category Type
#[derive(Debug, PartialEq, Serialize, Deserialize, TS, Clone)]
#[ts(export, export_to = "primitives.ts")]
pub enum WorkspaceMode {
    #[serde(rename = "LIVE")]
    Live,

    #[serde(rename = "DESIGN")]
    Design,
}

impl Default for WorkspaceMode {
    fn default() -> Self {
        Self::Live
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

impl ActivitybarPosition {
    pub const fn as_str(&self) -> &'static str {
        match self {
            ActivitybarPosition::Default => "DEFAULT",
            ActivitybarPosition::Top => "TOP",
            ActivitybarPosition::Bottom => "BOTTOM",
            ActivitybarPosition::Hidden => "HIDDEN",
        }
    }
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
