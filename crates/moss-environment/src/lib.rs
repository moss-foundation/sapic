pub mod builder;
pub mod configuration;
pub mod edit;
pub mod environment;
pub mod models;
mod segments;
pub mod utils;

pub use environment::Environment;

use moss_applib::AppRuntime;
use moss_bindingutils::primitives::ChangeString;
use std::path::PathBuf;

use crate::models::{
    primitives::{EnvironmentId, VariableId},
    types::{AddVariableParams, UpdateVariableParams, VariableInfo},
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
    pub name: Option<String>,
    pub color: Option<ChangeString>,
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}

pub struct DescribeEnvironment {
    pub id: EnvironmentId,
    pub name: String,
    pub color: Option<String>,
    pub variables: Vec<VariableInfo>,
    // TODO: git info
}

#[allow(private_bounds, async_fn_in_trait)]
pub trait AnyEnvironment<R: AppRuntime> {
    async fn abs_path(&self) -> PathBuf;
    async fn name(&self) -> joinerror::Result<String>;
    async fn describe(&self, ctx: &R::AsyncContext) -> joinerror::Result<DescribeEnvironment>;
    async fn modify(
        &self,
        ctx: &R::AsyncContext,
        params: ModifyEnvironmentParams,
    ) -> joinerror::Result<()>;
}
