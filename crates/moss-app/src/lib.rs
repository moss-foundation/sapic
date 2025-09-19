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
use moss_workspace::Workspace;

use crate::models::primitives::WorkspaceId;

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

pub mod constants {
    use moss_bindingutils::const_export;

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const ON_DID_CHANGE_CONFIGURATION_CHANNEL: &'static str = "app.onDidChangeConfiguration";

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const ON_DID_APPEND_LOG_ENTRY_CHANNEL: &'static str = "app.onDidAppendLogEntry";
}

pub mod dirs {
    pub const WORKSPACES_DIR: &str = "workspaces";
    pub const GLOBALS_DIR: &str = "globals";
    pub const PROFILES_DIR: &str = "profiles";
    pub const TMP_DIR: &str = "tmp";
}
