pub mod api;
pub mod collection;
pub mod config;
pub mod manifest;
pub mod models;
pub mod worktree;

pub use collection::Collection;

mod defaults {
    pub(crate) const DEFAULT_COLLECTION_NAME: &str = "New Collection";
}

pub mod dirs {
    pub const COLLECTIONS_DIR: &str = "requests";
    pub const ENVIRONMENTS_DIR: &str = "environments";
}
