use std::path::PathBuf;

use sapic_base::{
    project::types::primitives::ProjectId,
    resource::types::{ResourceSummary, primitives::*},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

//
// List Project Resources
//

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "main/types.ts")]
pub enum ListProjectResourcesMode {
    #[serde(rename = "LOAD_ROOT")]
    LoadRoot,
    #[serde(rename = "RELOAD_PATH")]
    ReloadPath(PathBuf),
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "main/operations.ts")]
pub struct ListProjectResourcesInput {
    pub project_id: ProjectId,
    pub mode: ListProjectResourcesMode,
}

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename = "ResourcePath", rename_all = "camelCase")]
#[ts(export, export_to = "main/types.ts")]
pub struct FrontendResourcePath {
    pub raw: PathBuf,
    pub segments: Vec<String>,
}

impl FrontendResourcePath {
    pub fn new(raw: PathBuf) -> Self {
        let segments = raw
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();

        Self { raw, segments }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "main/types.ts")]
pub struct ListProjectResourceItem {
    pub id: ResourceId,
    pub name: String,
    pub path: FrontendResourcePath,
    #[ts(type = "ResourceClass")]
    pub class: ResourceClass,
    #[ts(type = "ResourceKind")]
    pub kind: ResourceKind,
    #[ts(optional, type = "ResourceProtocol")]
    pub protocol: Option<ResourceProtocol>,
}

impl From<ResourceSummary> for ListProjectResourceItem {
    fn from(summary: ResourceSummary) -> Self {
        Self {
            id: summary.id,
            name: summary.name,
            path: FrontendResourcePath::new(summary.path),
            class: summary.class,
            kind: summary.kind,
            protocol: summary.protocol,
        }
    }
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "main/operations.ts")]
pub struct ListProjectResourcesOutput {
    #[ts(type = "ListProjectResourceItem[]")]
    pub items: Vec<ListProjectResourceItem>,
}
