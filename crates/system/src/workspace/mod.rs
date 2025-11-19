pub mod types;
pub mod workspace_service;

use async_trait::async_trait;
use std::sync::Arc;

use crate::workspace::types::*;

pub(super) type DynWorkspaceDiscoverer = Arc<dyn WorkspaceDiscoverer>;
#[async_trait]
pub trait WorkspaceDiscoverer: Send + Sync {
    async fn discover_workspaces(&self) -> joinerror::Result<Vec<DiscoveredWorkspace>>;
}
