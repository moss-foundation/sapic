use async_trait::async_trait;
use indexmap::IndexMap;
use moss_bindingutils::primitives::ChangeString;
use moss_environment::{
    DescribeEnvironment,
    configuration::VariableDecl,
    models::types::{AddVariableParams, UpdateVariableParams},
};
use moss_storage2::KvStorage;
use sapic_base::{
    environment::types::primitives::{EnvironmentId, VariableId},
    project::types::primitives::ProjectId,
    workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    environment::environment_service::CreateEnvironmentItemParams, project::LookedUpProject,
};

pub mod app_environment_service;
pub mod environment_edit_service;
pub mod environment_service;

pub struct CreateEnvironmentFsParams {
    pub project_id: Option<ProjectId>,
    pub name: String,
    pub color: Option<String>,
    pub variables: IndexMap<VariableId, VariableDecl>,
}

pub struct EnvironmentItemDescription {
    pub id: EnvironmentId,
    pub project_id: Option<ProjectId>,
    pub is_active: bool,
    pub display_name: String,
    pub order: Option<isize>,
    pub color: Option<String>,
    pub abs_path: Arc<Path>,
    pub total_variables: usize,
}

pub struct LookedUpEnvironment {
    pub id: EnvironmentId,
    // pub project_id: Option<ProjectId>,
    pub internal_abs_path: PathBuf,
}

#[async_trait]
pub trait EnvironmentServiceFs: Send + Sync {
    // async fn switch_workspace(
    //     &self,
    //     ctx: &dyn AnyAsyncContext,
    //     workspace_id: &WorkspaceId,
    // ) -> joinerror::Result<()>;

    // async fn add_source(
    //     &self,
    //     ctx: &dyn AnyAsyncContext,
    //     project_id: &ProjectId,
    //     source_path: &Path,
    // ) -> joinerror::Result<()>;
    //
    // async fn remove_source(
    //     &self,
    //     ctx: &dyn AnyAsyncContext,
    //     project_id: &ProjectId,
    // ) -> joinerror::Result<()>;

    async fn lookup_environments(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<LookedUpEnvironment>>;

    async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
        params: &CreateEnvironmentFsParams,
    ) -> joinerror::Result<PathBuf>;

    // HACK: Right now the environment file name is based on the environment name, which I think should be fixed
    // This means that we will need to pass in the full absolute path here

    async fn remove_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()>;
}

// This is only used by the app to create predefined environments after creating a workspace
// Since at that moment we don't immediately get the workspace handle
#[async_trait]
pub trait AppEnvironmentServiceFs: Send + Sync {
    async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        id: &EnvironmentId,
        params: &CreateEnvironmentFsParams,
    ) -> joinerror::Result<PathBuf>;
}

pub struct EnvironmentEditParams {
    pub name: Option<String>,
    pub color: Option<ChangeString>,
    pub vars_to_add: Vec<(VariableId, AddVariableParams)>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}

#[async_trait]
pub trait EnvironmentEditBackend: Send + Sync {
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: EnvironmentEditParams,
    ) -> joinerror::Result<()>;
}

// This is used by Welcome and Main window to create predefined environments when creating a new workspace
#[async_trait]
pub trait EnvironmentCreateOp: Send + Sync {
    async fn create(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<PathBuf>;
}
