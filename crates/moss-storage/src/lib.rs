pub mod collection_storage;
pub mod global_storage;
pub mod workspace_storage;

use std::sync::Arc;

use global_storage::*;
use workspace_storage::*;

pub trait GlobalStorage: Send + Sync {
    fn workspaces_store(&self) -> Arc<dyn WorkspacesStore>;
}

pub trait WorkspaceStorage: Send + Sync {
    fn collection_store(&self) -> Arc<dyn CollectionStore>;
    fn environment_store(&self) -> Arc<dyn EnvironmentStore>;
    fn state_store(&self) -> Arc<dyn StateStore>;
}

pub trait CollectionStorage: Send + Sync {}
