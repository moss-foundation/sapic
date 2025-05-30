pub mod api;
pub mod manifest;
pub mod models;
pub mod state;
pub mod storage;

pub mod workspace;
pub use workspace::Workspace;

pub mod constants {

    // ##################################################################
    // ###                                                            ###
    // ### !!! PLEASE UPDATE THE TYPESCRIPT CONSTANTS IN index.ts !!! ###
    // ###                                                            ###
    // ##################################################################

    // When adding/removing/modifying the TREE_VIEW_GROUP_* constants here,
    // you must also update the corresponding TypeScript constants
    // in moss-workspace/index.ts to match your changes.

    pub const TREE_VIEW_GROUP_COLLECTIONS: &str = "workbench.view.collections";
    pub const TREE_VIEW_GROUP_ENVIRONMENTS: &str = "workbench.view.environments";
    pub const TREE_VIEW_GROUP_MOCK_SERVERS: &str = "workbench.view.mockServers";
}

mod defaults {
    use super::constants::*;

    pub const DEFAULT_TREE_VIEW_GROUP: &str = TREE_VIEW_GROUP_COLLECTIONS;
    pub(crate) const DEFAULT_WORKSPACE_NAME: &str = "New Workspace";
}

pub mod dirs {
    pub const COLLECTIONS_DIR: &str = "collections";
    pub const ENVIRONMENTS_DIR: &str = "environments";
}
