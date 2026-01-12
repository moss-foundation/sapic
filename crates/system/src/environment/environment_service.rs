use async_trait::async_trait;
use indexmap::IndexMap;
use joinerror::ResultExt;
use moss_common::continue_if_err;
use moss_environment::{
    DescribeEnvironment,
    builder::{CreateEnvironmentParams, EnvironmentBuilder},
    configuration::VariableDecl,
    models::types::AddVariableParams,
    storage::{key_environment, key_variable_local_value},
};
use moss_fs::FileSystem;
use moss_hcl::json_to_hcl;
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use sapic_base::{
    environment::types::primitives::{EnvironmentId, VariableId},
    project::types::primitives::ProjectId,
    workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::environment::{
    CreateEnvironmentFsParams, EnvironmentInitializeOp, EnvironmentServiceFs,
};

pub struct CreateEnvironmentItemParams {
    pub env_id: EnvironmentId,
    pub project_id: Option<ProjectId>,
    pub name: String,
    pub order: isize,
    pub color: Option<String>,
    pub variables: Vec<AddVariableParams>,
}

pub struct EnvironmentItem {
    pub id: EnvironmentId,
    pub project_id: Option<ProjectId>,
    pub internal_abs_path: PathBuf,
}

pub struct EnvironmentService {
    backend: Arc<dyn EnvironmentServiceFs>,
    storage: Arc<dyn KvStorage>,
}

impl EnvironmentService {
    pub fn new(backend: Arc<dyn EnvironmentServiceFs>, storage: Arc<dyn KvStorage>) -> Self {
        Self { backend, storage }
    }

    // HACK: I'm not sure the best way to handle environment sources when switching to a new workspace
    // This method is used when a new workspace is open, during which the old sources are cleared and
    // the workspace's base source is added (projects sources will be added during discovery)
    pub async fn open_workspace(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
    ) -> joinerror::Result<()> {
        self.backend.switch_workspace(ctx, workspace_id).await
    }

    pub async fn add_source(
        &self,
        ctx: &dyn AnyAsyncContext,
        project_id: &ProjectId,
        source_path: &Path,
    ) -> joinerror::Result<()> {
        self.backend.add_source(ctx, project_id, source_path).await
    }

    pub async fn remove_source(
        &self,
        ctx: &dyn AnyAsyncContext,
        project_id: &ProjectId,
    ) -> joinerror::Result<()> {
        self.backend.remove_source(ctx, project_id).await
    }

    pub async fn environments(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<EnvironmentItem>> {
        let discovered_environments = self
            .backend
            .lookup_environments(ctx)
            .await
            .join_err::<()>("failed to lookup environments")?;

        let environments = discovered_environments
            .into_iter()
            .map(|env| EnvironmentItem {
                id: env.id,
                project_id: env.project_id,
                internal_abs_path: env.internal_abs_path,
            })
            .collect::<Vec<_>>();
        Ok(environments)
    }

    // This is primarily used to initialize predefined environments, when the storage and workspace are not yet prepared.
    pub async fn initialize_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<PathBuf> {
        let mut variable_decls = IndexMap::new();
        for param in params.variables {
            let id = VariableId::new();
            let global_value = continue_if_err!(json_to_hcl(&param.global_value), |err| {
                println!("failed to convert global value expression: {}", err); // TODO: log error
            });
            let decl = VariableDecl {
                name: param.name,
                value: global_value,
                description: param.desc,
                options: param.options,
            };
            variable_decls.insert(id, decl);
        }

        let abs_path = self
            .backend
            .initialize_environment(
                ctx,
                workspace_id,
                &params.env_id,
                &CreateEnvironmentFsParams {
                    project_id: params.project_id,
                    name: params.name,
                    color: params.color,
                    variables: variable_decls,
                },
            )
            .await?;

        Ok(abs_path)
    }

    pub async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<EnvironmentItem> {
        let mut variable_decls = IndexMap::new();
        let mut variable_localvalues = HashMap::new();
        for param in params.variables {
            let id = VariableId::new();
            let global_value = continue_if_err!(json_to_hcl(&param.global_value), |err| {
                println!("failed to convert global value expression: {}", err); // TODO: log error
            });
            let decl = VariableDecl {
                name: param.name,
                value: global_value,
                description: param.desc,
                options: param.options,
            };
            variable_decls.insert(id.clone(), decl);
            variable_localvalues.insert(id.clone(), param.local_value.clone());
        }

        let id = EnvironmentId::new();
        let internal_abs_path = self
            .backend
            .initialize_environment(
                ctx,
                workspace_id,
                &id,
                &CreateEnvironmentFsParams {
                    project_id: params.project_id.clone(),
                    name: params.name,
                    color: params.color,
                    variables: variable_decls,
                },
            )
            .await?;

        // The signature of storage.put_batch makes it tricky to use it here
        // I'll try to figure this out later
        for (var_id, local_value) in variable_localvalues {
            let local_value_key = key_variable_local_value(&id, &var_id);

            if let Err(e) = self
                .storage
                .put(
                    ctx,
                    StorageScope::Workspace(workspace_id.inner()),
                    &local_value_key,
                    local_value,
                )
                .await
            {
                tracing::error!("failed to store local value: {}", e);
            }
        }

        Ok(EnvironmentItem {
            id,
            project_id: params.project_id,
            internal_abs_path,
        })
    }
    // HACK: we need the description to properly clean up all the variables from the storage
    // Maybe there's a better way to structure this
    pub async fn delete_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        desc: DescribeEnvironment,
    ) -> joinerror::Result<()> {
        self.backend
            .remove_environment(ctx, desc.abs_path.as_ref())
            .await?;

        {
            let storage_scope = StorageScope::Workspace(workspace_id.inner());
            // Clean all the metadata and variables related to the deleted environment
            if let Err(e) = self
                .storage
                .remove_batch_by_prefix(ctx, storage_scope.clone(), &key_environment(&desc.id))
                .await
            {
                tracing::warn!("failed to remove environment data from the db: {}", e);
            }
        }

        Ok(())
    }
}

#[async_trait]
impl EnvironmentInitializeOp for EnvironmentService {
    async fn initialize(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<PathBuf> {
        self.initialize_environment(ctx, workspace_id, params).await
    }
}
