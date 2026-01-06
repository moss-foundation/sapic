pub mod api;
pub mod builder;
pub mod contribution;
mod edit;
pub mod git;
pub mod models;
pub mod project;
pub mod registries;
pub mod vcs;
mod worktree;

mod set_icon;
#[cfg(feature = "integration-tests")]
pub mod storage;
#[cfg(not(feature = "integration-tests"))]
mod storage;

pub use builder::ProjectBuilder;
pub use project::{Project, ProjectModifyParams};

use crate::contribution::{RegisterHttpHeadersContribution, RegisterResourceStatusesContribution};

inventory::submit! {
    RegisterResourceStatusesContribution(include_str!(concat!(env!("OUT_DIR"), "/resourceStatuses.json")))
}

inventory::submit! {
    RegisterHttpHeadersContribution(include_str!(concat!(env!("OUT_DIR"), "/httpHeaders.json")))
}

pub mod constants {
    pub const ITEM_CONFIG_FILENAME: &str = "config.sap";
    pub const DIR_CONFIG_FILENAME: &str = "config-folder.sap";
}

pub mod dirs {
    pub const RESOURCES_DIR: &str = "resources";
    pub const ENVIRONMENTS_DIR: &str = "environments";
    pub const ASSETS_DIR: &str = "assets";
}

pub mod errors {
    use joinerror::error::ErrorMarker;

    pub struct ErrorInvalidInput;
    impl ErrorMarker for ErrorInvalidInput {
        const MESSAGE: &'static str = "invalid_input";
    }

    pub struct ErrorInvalidKind;
    impl ErrorMarker for ErrorInvalidKind {
        const MESSAGE: &'static str = "invalid_kind";
    }

    pub struct ErrorAlreadyExists;
    impl ErrorMarker for ErrorAlreadyExists {
        const MESSAGE: &'static str = "already_exists";
    }

    pub struct ErrorNotFound;
    impl ErrorMarker for ErrorNotFound {
        const MESSAGE: &'static str = "not_found";
    }

    pub struct ErrorIo;
    impl ErrorMarker for ErrorIo {
        const MESSAGE: &'static str = "io";
    }

    pub struct ErrorInternal;
    impl ErrorMarker for ErrorInternal {
        const MESSAGE: &'static str = "internal";
    }

    pub struct ErrorUnknown;
    impl ErrorMarker for ErrorUnknown {
        const MESSAGE: &'static str = "unknown";
    }
}
