pub mod api;
pub mod builder;
pub mod collection;
pub mod config;
pub mod context;
pub mod manifest;
pub mod models;
pub mod services;
pub mod storage;

pub use builder::CollectionBuilder;
pub use collection::{Collection, CollectionModifyParams};

pub mod constants {
    pub const COLLECTION_ROOT_PATH: &str = "";

    pub const ITEM_CONFIG_FILENAME: &str = "config.sapic";
    pub const DIR_CONFIG_FILENAME: &str = "config-folder.sapic";
}

mod defaults {
    pub(crate) const DEFAULT_COLLECTION_NAME: &str = "New Collection";
    pub(crate) const _DEFAULT_ENDPOINT_NAME: &str = "New Endpoint";
    pub(crate) const _DEFAULT_COMPONENT_NAME: &str = "New Component";
    pub(crate) const _DEFAULT_SCHEMA_NAME: &str = "New Schema";
    pub(crate) const _DEFAULT_ENVIRONMENT_NAME: &str = "New Environment";
}

// When updating this, the `validate_input_path` method in models/operations.rs
// should also be updated
pub mod dirs {
    pub const REQUESTS_DIR: &str = "requests";
    pub const ENDPOINTS_DIR: &str = "endpoints";
    pub const COMPONENTS_DIR: &str = "components";
    pub const SCHEMAS_DIR: &str = "schemas";
    pub const ENVIRONMENTS_DIR: &str = "environments";
    pub const ASSETS_DIR: &str = "assets";
}
