use sapic_base::workspace::types::primitives::WorkspaceId;
use std::{path::Path, sync::Arc};

pub struct WorkspaceItem {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: Arc<Path>, // TODO: do we need Arc here?
    pub last_opened_at: Option<i64>,
}
