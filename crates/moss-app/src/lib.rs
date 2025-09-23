pub mod api;
pub mod app;
pub mod builder;
pub mod command;
mod configuration;
mod internal;
mod locale;
mod logging;
pub mod models;
mod profile;
mod session;
mod theme;
mod workspace;

#[cfg(feature = "integration-tests")]
pub mod storage;
#[cfg(not(feature = "integration-tests"))]
mod storage;

#[macro_use]
extern crate derive_more;

pub use app::App;
pub use builder::AppBuilder;
use moss_applib::AppRuntime;
use moss_configuration::IncludeConfigurationDecl;
use moss_workspace::Workspace;

use crate::models::primitives::WorkspaceId;

inventory::submit! {
    IncludeConfigurationDecl(include_str!(concat!(env!("OUT_DIR"), "/", env!("CARGO_PKG_NAME"), ".contrib.json")))
}

#[derive(Deref, DerefMut)]
pub struct ActiveWorkspace<R: AppRuntime> {
    id: WorkspaceId,

    #[deref]
    #[deref_mut]
    handle: Workspace<R>,
}

impl<R: AppRuntime> ActiveWorkspace<R> {
    pub fn id(&self) -> WorkspaceId {
        self.id.clone()
    }
}

#[rustfmt::skip]
pub mod constants {
    use moss_bindingutils::const_export;

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const ON_DID_CHANGE_CONFIGURATION_CHANNEL: &'static str = "app__on_did_change_configuration";

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const ON_DID_APPEND_LOG_ENTRY_CHANNEL: &'static str = "app__on_did_append_log_entry";
}

pub mod dirs {
    pub const WORKSPACES_DIR: &str = "workspaces";
    pub const GLOBALS_DIR: &str = "globals";
    pub const PROFILES_DIR: &str = "profiles";
    pub const TMP_DIR: &str = "tmp";
}

// /// Generated configuration from Jsonnet at build time
// pub const GENERATED_CONFIG_JSON: &str =
//     include_str!(concat!(env!("OUT_DIR"), "/generated_config.json"));
