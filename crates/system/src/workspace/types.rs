use sapic_base::workspace::types::primitives::WorkspaceId;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct DiscoveredWorkspace {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: PathBuf,
}

pub struct KnownWorkspace {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: Arc<Path>, // TODO: do we need Arc here?
    pub last_opened_at: Option<i64>,
}
