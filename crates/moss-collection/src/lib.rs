pub mod api;
pub mod collection;
pub mod kdl;
pub mod manifest;
pub mod models;
pub mod worktree;

mod defaults {
    pub(crate) const DEFAULT_COLLECTION_NAME: &str = "New Collection";
}

mod dirs {
    pub(crate) const COLLECTIONS_DIR: &str = "requests";
    pub(crate) const ENVIRONMENTS_DIR: &str = "environments";
}
