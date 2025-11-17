pub mod api;
pub mod builder;
mod edit;
mod environment;
mod layout;
mod manifest;
pub mod models;
mod project;

pub mod storage;

// FIXME: Remove it once we switch environment to new db
#[cfg(not(feature = "integration-tests"))]
mod storage_old;
#[cfg(feature = "integration-tests")]
pub mod storage_old;
pub mod workspace;

use moss_applib::AppRuntime;
use moss_environment::AnyEnvironment;

pub use workspace::Workspace;

pub mod constants {
    use moss_bindingutils::const_export;

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const TREE_VIEW_GROUP_PROJECTS: &str = "workbench.view.projects";

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const TREE_VIEW_GROUP_ENVIRONMENTS: &str = "workbench.view.environments";

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const TREE_VIEW_GROUP_MOCK_SERVERS: &str = "workbench.view.mockServers";
}

pub mod dirs {
    pub const PROJECTS_DIR: &str = "projects";
    pub const ENVIRONMENTS_DIR: &str = "environments";
}

pub trait AnyWorkspace<R: AppRuntime> {
    type Project;
    type Environment: AnyEnvironment<R>;
}

pub mod errors {
    use joinerror::error::ErrorMarker;

    pub struct ErrorNotFound;
    impl ErrorMarker for ErrorNotFound {
        const MESSAGE: &'static str = "not_found";
    }
}
