pub mod api;
pub mod manifest;
pub mod models;
pub mod storage;

pub mod workspace;
pub use workspace::Workspace;

mod defaults {
    pub(crate) const DEFAULT_WORKSPACE_NAME: &str = "New Workspace";
}

pub mod dirs {
    pub const COLLECTIONS_DIR: &str = "collections";
    pub const ENVIRONMENTS_DIR: &str = "environments";
}
