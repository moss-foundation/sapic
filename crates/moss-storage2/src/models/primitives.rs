use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum StorageScope {
    /// The stored data will be scoped globally.
    Application,

    /// The stored data will be scoped to a specific workspace.
    Workspace(Arc<String>),

    /// The stored data will be scoped to a specific project.
    Project(Arc<String>),
}
