use sapic_base::resource::types::primitives::ResourceId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{
    primitives::{FrontendResourcePath, ResourceClass, ResourceKind, ResourceProtocol},
    types::{AfterUpdateDirResourceDescription, AfterUpdateItemResourceDescription},
};

/// @category Event
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "events.ts")]
pub struct StreamResourcesEvent {
    /// Unique identifier for this resource
    pub id: ResourceId,

    /// Display name of the resource
    pub name: String,

    /// Path relative to the project root.
    /// Includes both the original path string and its segments.
    pub path: FrontendResourcePath,

    /// Classification of the resource (Endpoint, Component, or Schema)
    pub class: ResourceClass,

    /// Type of resource indicating its structure (Dir for directories, Item for files, Case of item cases)
    pub kind: ResourceKind,

    /// HTTP protocol/method used by this resource, if applicable (GET, POST, PUT, DELETE, WebSocket, GraphQL, gRPC)
    pub protocol: Option<ResourceProtocol>,

    /// Determines the display position of this resource among others in the same group.
    /// Resources are sorted in ascending order; lower values appear before higher ones.
    /// Negative values are allowed and will be placed before positive values.
    /// If multiple resources have the same order, they are sorted alphabetically.
    /// If not specified, the resource appears last and is sorted alphabetically
    /// among unspecified items.
    pub order: Option<isize>,

    /// Whether this resource is expanded in the tree view (applies to directories)
    pub expanded: bool,
}

/// @category Event
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "events.ts")]
pub enum BatchUpdateResourceEvent {
    Item(AfterUpdateItemResourceDescription),
    Dir(AfterUpdateDirResourceDescription),
}
