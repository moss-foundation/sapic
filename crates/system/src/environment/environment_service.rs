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
use moss_hcl::{hcl_to_json, json_to_hcl};
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use sapic_base::{
    environment::types::{
        VariableInfo,
        primitives::{EnvironmentId, VariableId},
    },
    project::types::primitives::ProjectId,
    workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::environment::{CreateEnvironmentFsParams, EnvironmentCreateOp, EnvironmentServiceFs};

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

// Both Workspace and Project has their own EnvironmentService
// FIXME: Should they both store variables in the Workspace storage scope?

pub struct EnvironmentService {
    workspace_id: WorkspaceId,
    project_id: Option<ProjectId>,
    backend: Arc<dyn EnvironmentServiceFs>,
    storage: Arc<dyn KvStorage>,
}

impl EnvironmentService {
    pub fn new(
        workspace_id: WorkspaceId,
        project_id: Option<ProjectId>,
        backend: Arc<dyn EnvironmentServiceFs>,
        storage: Arc<dyn KvStorage>,
    ) -> Self {
        Self {
            workspace_id,
            project_id,
            backend,
            storage,
        }
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
                project_id: self.project_id.clone(),
                internal_abs_path: env.internal_abs_path,
            })
            .collect::<Vec<_>>();
        Ok(environments)
    }

    pub async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
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
            .create_environment(
                ctx,
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
                    StorageScope::Workspace(self.workspace_id.inner()),
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
    pub async fn delete_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()> {
        self.backend.remove_environment(ctx, id).await?;

        // FIXME: Should the scope be project if the environment is a project level environment?
        let storage_scope = StorageScope::Workspace(self.workspace_id.inner());
        // Clean all the metadata and variables related to the deleted environment
        if let Err(e) = self
            .storage
            .remove_batch_by_prefix(ctx, storage_scope.clone(), &key_environment(id))
            .await
        {
            tracing::warn!("failed to remove environment data from the db: {}", e);
        }

        Ok(())
    }

    pub async fn describe_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<DescribeEnvironment> {
        let parsed = self.backend.read_environment_sourcefile(ctx, id).await?;
        let mut variables =
            HashMap::with_capacity(parsed.variables.as_ref().map_or(0, |v| v.len()));

        if let Some(vars) = parsed.variables.as_ref() {
            // TODO: Use project storage scope for project environments
            let storage_scope = StorageScope::Workspace(self.workspace_id.inner());

            for (var_id, var) in vars.iter() {
                let global_value = continue_if_err!(hcl_to_json(&var.value), |err| {
                    println!("failed to convert global value expression: {}", err); // TODO: log error
                });

                let local_value: Option<JsonValue> = self
                    .storage
                    .get(
                        ctx,
                        storage_scope.clone(),
                        &key_variable_local_value(id, var_id),
                    )
                    .await
                    .unwrap_or_else(|e| {
                        tracing::warn!(
                            "failed to get variable localValue from the database: {}",
                            e
                        );
                        None
                    });

                // FIXME: Should the variables be cached?
                variables.insert(
                    var_id.clone(),
                    VariableInfo {
                        id: var_id.clone(),
                        name: var.name.clone(),
                        global_value: Some(global_value),
                        local_value,
                        disabled: var.options.disabled,
                        order: None, // TODO: REMOVE
                        desc: var.description.clone(),
                    },
                );
            }
        }
        Ok(DescribeEnvironment {
            id: id.clone(),
            name: parsed.metadata.name.clone(),
            color: parsed.metadata.color.clone(),
            variables,
        })
    }
}
