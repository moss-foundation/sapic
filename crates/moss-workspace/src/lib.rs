pub mod api;
pub mod builder;
mod edit;
mod environment;
mod layout;
mod manifest;
pub mod models;
mod project;
pub mod storage;
pub mod workspace;

use moss_applib::AppRuntime;
use moss_configuration::{ConfigurationDecl, ParameterDecl, models::primitives::ParameterType};
use moss_environment::AnyEnvironment;
use moss_text::read_only_str;

pub use workspace::Workspace;

inventory::submit! {
    ConfigurationDecl {
        id: Some(read_only_str!("moss_workspace")),
        parent_id: None,
        name: Some("Workspace"),
        order: Some(1),
        description: Some("Workspace configuration"),
        parameters: &[
            ParameterDecl {
                id: read_only_str!("name"),
                default: Some(static_json::Value::Str("Hello, World!")),
                typ: ParameterType::String,
                description: None,
                maximum: None,
                minimum: None,
                excluded: false,
                protected: false,
                order: None,
                tags: &[],
            },
        ],
    }
}

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
