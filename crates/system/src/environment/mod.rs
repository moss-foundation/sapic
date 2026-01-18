use crate::environment::environment_service::CreateEnvironmentItemParams;
use async_trait::async_trait;
use indexmap::IndexMap;
use moss_bindingutils::primitives::ChangeString;
use moss_environment::{
    configuration::{SourceFile, VariableDecl},
    models::types::{AddVariableParams, UpdateVariableParams},
};
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

pub mod environment_edit_service;
pub mod environment_service;

pub struct CreateEnvironmentFsParams {
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
    async fn lookup_environments(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<LookedUpEnvironment>>;

    async fn read_environment_sourcefile(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<SourceFile>;

    async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
        params: &CreateEnvironmentFsParams,
    ) -> joinerror::Result<PathBuf>;

    // This method is the only one used by app-level environment service
    // It's used to create predefined environments for newly created workspaces
    async fn create_workspace_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        id: &EnvironmentId,
        params: &CreateEnvironmentFsParams,
    ) -> joinerror::Result<PathBuf>;

    async fn remove_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()>;
}

#[derive(Clone)]
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
pub trait WorkspaceEnvironmentCreateOp: Send + Sync {
    async fn create(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<PathBuf>;
}
