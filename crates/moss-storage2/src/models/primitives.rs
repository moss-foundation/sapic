use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum StorageScope {
    /// The stored data will be scoped to all workspaces across all profiles.
    Application,

    /// The stored data will be scoped to a specific workspace.
    Workspace(Arc<String>),

    /// The stored data will be scoped to a specific collection.
    Collection(Arc<String>),
}
