pub mod api;
pub mod collection;
pub mod config;
pub mod context;
pub mod manifest;
pub mod models;
pub mod services;
pub mod storage;
pub mod worktree;

pub use collection::Collection;

pub mod constants {
    pub const COLLECTION_ICON_FILENAME: &str = "icon.png";
    pub const ITEM_CONFIG_FILENAME: &str = "config.sapic";
    pub const DIR_CONFIG_FILENAME: &str = "config-folder.sapic";

    pub const ID_LENGTH: usize = 10;
}

mod defaults {
    pub(crate) const DEFAULT_COLLECTION_NAME: &str = "New Collection";
    pub(crate) const _DEFAULT_ENDPOINT_NAME: &str = "New Endpoint";
    pub(crate) const _DEFAULT_COMPONENT_NAME: &str = "New Component";
    pub(crate) const _DEFAULT_SCHEMA_NAME: &str = "New Schema";
    pub(crate) const _DEFAULT_ENVIRONMENT_NAME: &str = "New Environment";
}

pub mod dirs {
    pub const REQUESTS_DIR: &str = "requests";
    pub const ENDPOINTS_DIR: &str = "endpoints";
    pub const COMPONENTS_DIR: &str = "components";
    pub const SCHEMAS_DIR: &str = "schemas";
    pub const ENVIRONMENTS_DIR: &str = "environments";
    pub const ASSETS_DIR: &str = "assets";
}
