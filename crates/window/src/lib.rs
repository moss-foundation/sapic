pub mod api;
pub mod builder;

mod logging;
pub mod models;
mod profile;
mod session;
pub mod window;
mod workspace;

#[cfg(feature = "integration-tests")]
pub mod storage;
#[cfg(not(feature = "integration-tests"))]
mod storage;

pub use builder::OldSapicWindowBuilder;
pub use window::OldSapicWindow;

#[rustfmt::skip]
pub mod constants {
    use moss_bindingutils::const_export;

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const ON_DID_CHANGE_CONFIGURATION_CHANNEL: &'static str = "app__on_did_change_configuration";

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const ON_DID_APPEND_LOG_ENTRY_CHANNEL: &'static str = "app__on_did_append_log_entry";

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const ON_DID_ADD_EXTENSION_CHANNEL: &'static str = "app__on_did_add_extension";

}

pub mod dirs {
    pub const WORKSPACES_DIR: &str = "workspaces";
    pub const GLOBALS_DIR: &str = "globals";
    pub const PROFILES_DIR: &str = "profiles";
    pub const TMP_DIR: &str = "tmp";
}
