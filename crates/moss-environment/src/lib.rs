pub mod builder;
pub mod configuration;
pub mod environment;
pub mod models;
pub mod registry;
pub mod services;

pub use environment::Environment;
pub use registry::GlobalEnvironmentRegistry;

use moss_applib::AppRuntime;
use moss_bindingutils::primitives::ChangeString;

use crate::{
    models::{
        primitives::VariableId,
        types::{AddVariableParams, UpdateVariableParams},
    },
    services::{AnyMetadataService, AnyStorageService, AnySyncService, AnyVariableService},
};

pub mod constants {
    pub const ENVIRONMENT_FILE_EXTENSION: &str = "env.sap";
}

pub mod errors {
    use joinerror::error::ErrorMarker;

    pub struct ErrorEnvironmentAlreadyExists;
    impl ErrorMarker for ErrorEnvironmentAlreadyExists {
        const MESSAGE: &'static str = "already_exists";
    }

    pub struct ErrorEnvironmentNotFound;
    impl ErrorMarker for ErrorEnvironmentNotFound {
        const MESSAGE: &'static str = "not_found";
    }

    pub struct ErrorFailedToEncode;
    impl ErrorMarker for ErrorFailedToEncode {
        const MESSAGE: &'static str = "failed_to_encode";
    }

    pub struct ErrorFailedToDecode;
    impl ErrorMarker for ErrorFailedToDecode {
        const MESSAGE: &'static str = "failed_to_decode";
    }

    pub struct ErrorIo;
    impl ErrorMarker for ErrorIo {
        const MESSAGE: &'static str = "io";
    }
}

pub struct ModifyEnvironmentParams {
    pub color: Option<ChangeString>,
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}

#[allow(private_bounds, async_fn_in_trait)]
pub trait AnyEnvironment<R: AppRuntime> {
    type StorageService: AnyStorageService<R>;
    type VariableService: AnyVariableService<R>;
    type SyncService: AnySyncService<R>;
    type MetadataService: AnyMetadataService<R>;

    async fn modify(&self, params: ModifyEnvironmentParams) -> joinerror::Result<()>;
}
