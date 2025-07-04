pub mod api;
pub mod builder;
pub mod context;
pub mod manifest;
pub mod models;
pub mod services;
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

pub mod dirs {
    pub const COLLECTIONS_DIR: &str = "collections";
    pub const ENVIRONMENTS_DIR: &str = "environments";
}
