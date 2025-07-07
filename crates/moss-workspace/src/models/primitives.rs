use derive_more::Deref;
use nanoid::nanoid;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CollectionId(Arc<String>);
impl CollectionId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for CollectionId {
    fn from(s: String) -> Self {
        CollectionId(Arc::new(s))
    }
}

impl AsRef<str> for CollectionId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

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
