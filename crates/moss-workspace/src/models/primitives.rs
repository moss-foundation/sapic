use derive_more::Deref;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

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
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ChangeCollectionId {
    Update(CollectionId),
    Remove,
}

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CollectionId(Arc<String>);
impl CollectionId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }

    pub fn inner(self) -> Arc<String> {
        self.0
    }
}

impl From<String> for CollectionId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl From<Arc<String>> for CollectionId {
    fn from(s: Arc<String>) -> Self {
        Self(s)
    }
}

impl AsRef<str> for CollectionId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[rustfmt::skip]
impl TS for CollectionId {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name() -> String { "string".to_string() }
    fn inline() -> String { "string".to_string() }
    fn inline_flattened() -> String { "string".to_string() }
    fn decl() -> String { unreachable!() }
    fn decl_concrete() -> String { unreachable!() }
    fn dependencies() -> Vec<ts_rs::Dependency> { vec![] }
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
