pub mod api;
pub mod builder;
pub mod edit;
pub mod manifest;
pub mod models;
pub mod services;
pub mod storage;
pub mod workspace;

use moss_applib::AppRuntime;
use moss_environment::AnyEnvironment;
pub use workspace::Workspace;

pub mod constants {
    use moss_bindingutils::const_export;

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
    pub const TREE_VIEW_GROUP_COLLECTIONS: &str = "workbench.view.collections";

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
