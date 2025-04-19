pub mod collection_storage;
pub mod global_storage;
pub mod workspace_storage;

use std::sync::Arc;

use global_storage::WorkspacesStore;

pub trait GlobalStorage: Send + Sync {
    fn workspaces_store(&self) -> Arc<dyn WorkspacesStore>;
}

pub trait WorkspaceStorage: Send + Sync {}

pub trait CollectionStorage: Send + Sync {}
