pub mod api;
pub mod manifest;
pub mod models;
pub mod storage;

mod workspace;
pub use workspace::*;

mod defaults {
    pub(crate) const DEFAULT_WORKSPACE_NAME: &str = "New Workspace";
}

mod dirs {
    pub(crate) const COLLECTIONS_DIR: &str = "collections";
    pub(crate) const ENVIRONMENTS_DIR: &str = "environments";
}
