use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type CollectionId = String;

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS, Validate)]
// #[serde(transparent)]
// #[ts(export, export_to = "primitives.ts")]
// pub struct GitUrl {
//     #[validate(regex(path = *RE_TWO_CHARS))]
//     pub url: String,
// }

// impl GitUrl {
//     pub fn new(url: String) -> Self {
//         Self { url }
//     }

//     pub fn as_str(&self) -> &str {
//         &self.url
//     }

//     pub fn as_string(&self) -> String {
//         self.url.clone()
//     }
// }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum SidebarPosition {
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum EditorGridOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub enum PanelRenderer {
    OnlyWhenVisible,
    Always,
}
