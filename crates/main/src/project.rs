use derive_more::Deref;
use joinerror::{OptionExt, ResultExt};
use moss_bindingutils::primitives::ChangeJsonValue;
use moss_environment::{
    DescribeEnvironment,
    builder::{CreateEnvironmentParams, EnvironmentBuilder, EnvironmentLoadParams},
    storage::{key_variable, key_variable_local_value},
};
use moss_fs::FileSystem;
use moss_project::Project;
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use moss_workspace::storage::KEY_ACTIVE_ENVIRONMENT;
use rustc_hash::FxHashMap;
use sapic_base::{
    environment::types::primitives::{EnvironmentId, VariableId},
    project::types::primitives::ProjectId,
    resource::types::ResourceSummary,
    workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use sapic_ipc::contracts::main::environment::{CreateEnvironmentInput, UpdateEnvironmentParams};
use sapic_platform::environment::environment_edit_backend::EnvironmentFsEditBackend;
use sapic_system::{
    environment::{
        EnvironmentEditParams, EnvironmentItemDescription,
        environment_edit_service::EnvironmentEditService,
        environment_service::{CreateEnvironmentItemParams, EnvironmentService},
    },
    project::project_edit_service::ProjectEditService,
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{OnceCell, RwLock, mpsc};

use crate::environment::RuntimeEnvironment;

#[derive(Deref)]
pub struct RuntimeProject {
    pub id: ProjectId,
    pub workspace_id: WorkspaceId,

    #[deref]
    pub handle: Arc<Project>,

    pub(crate) edit: ProjectEditService,
    pub(crate) storage: Arc<dyn KvStorage>,
    pub(crate) fs: Arc<dyn FileSystem>,

    pub(crate) environment_service: Arc<EnvironmentService>,
    pub(crate) environments: OnceCell<RwLock<FxHashMap<EnvironmentId, RuntimeEnvironment>>>,
    pub(crate) active_environment: RwLock<Option<EnvironmentId>>,
}

impl RuntimeProject {
    async fn environments_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<&RwLock<FxHashMap<EnvironmentId, RuntimeEnvironment>>> {
        self.environments
            .get_or_try_init(|| async {
                let active_environment_result = self
                    .storage
                    .get(
                        ctx,
                        StorageScope::Project(self.id.inner()),
                        KEY_ACTIVE_ENVIRONMENT,
                    )
                    .await;

                let active_environment: Option<EnvironmentId> = match active_environment_result {
                    Ok(Some(active_environment)) => {
                        serde_json::from_value(active_environment).unwrap_or_default()
                    }
                    Ok(None) => None,
                    Err(e) => {
                        tracing::warn!(
                            "failed to get the active environment for project {}: {}",
                            self.id.to_string(),
                            e
                        );
                        None
                    }
                };

                let mut active_environment_lock = self.active_environment.write().await;
                *active_environment_lock = active_environment;

                let environments = self.environment_service.environments(ctx).await?;

                let mut result = FxHashMap::default();

                for environment in environments {
                    let env_id = environment.id;
                    let builder = EnvironmentBuilder::new(
                        self.workspace_id.inner(),
                        self.fs.clone(),
                        self.storage.clone(),
                        env_id.clone(),
                    );

                    let handle = builder
                        .load(EnvironmentLoadParams {
                            abs_path: environment.internal_abs_path.clone(),
                        })
                        .await?;

                    result.insert(
                        env_id.clone(),
                        RuntimeEnvironment {
                            id: env_id.clone(),
                            project_id: environment.project_id.clone(),
                            handle: handle.into(),
                            edit: EnvironmentEditService::new(EnvironmentFsEditBackend::new(
                                &environment.internal_abs_path,
                                self.fs.clone(),
                            )),
                        },
                    );
                }

                Ok::<_, joinerror::Error>(RwLock::new(result))
            })
            .await
            .join_err::<()>("failed to get project environments")
    }

    async fn environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<RuntimeEnvironment> {
        let environments = self.environments_internal(ctx).await?;
        let environment = environments
            .read()
            .await
            .get(id)
            .cloned()
            .ok_or_join_err_with::<()>(|| format!("environment {} not found", id.to_string()))?;

        Ok(environment)
    }
}
impl RuntimeProject {
    // FIXME: Right now, the method for getting projects and the one for getting environments return conceptually different types.
    // Most likely, both methods will need to be adjusted to return Runtime types.
    pub async fn resources(
        &self,
        ctx: &dyn AnyAsyncContext,
        dirs: Vec<PathBuf>,
    ) -> joinerror::Result<Vec<ResourceSummary>> {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let worktree = self.handle.worktree().await.clone();

        let mut handles = Vec::new();
        for dir in dirs {
            let tx_clone = tx.clone();
            let worktree_clone = worktree.clone();
            let ctx_clone = ctx.clone_arc();

            handles.push(tokio::spawn(async move {
                worktree_clone
                    .scan(ctx_clone, dir.as_path(), tx_clone)
                    .await
            }));
        }
        drop(tx);

        let mut items = vec![];
        while let Some(item) = rx.recv().await {
            items.push(ResourceSummary {
                id: item.id,
                name: item.name,
                path: item.path.to_path_buf(),
                class: item.class,
                kind: item.kind,
                protocol: item.protocol,
            });
        }

        for result in futures::future::join_all(handles).await {
            let _ = result.map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;
        }

        Ok(items)
    }

    pub async fn environments(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<RuntimeEnvironment>> {
        let environments = self.environments_internal(ctx).await?;

        Ok(environments.read().await.clone().into_values().collect())
    }

    pub async fn active_environment(
        &self,
        _ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Option<EnvironmentId>> {
        let active_environments = self.active_environment.read().await;

        Ok(active_environments.clone())
    }

    pub async fn describe_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<DescribeEnvironment> {
        self.environment_service.describe_environment(ctx, id).await
    }

    pub async fn activate_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()> {
        let environments = self.environments_internal(ctx).await?.read().await;

        let _environment_item = environments
            .get(&id)
            .ok_or_join_err_with::<()>(|| format!("environment {} not found", id.to_string()))?;

        let mut active_environment = self.active_environment.write().await;
        *active_environment = Some(id.clone());
        if let Err(e) = self
            .storage
            .put(
                ctx,
                StorageScope::Project(self.id.inner()),
                KEY_ACTIVE_ENVIRONMENT,
                serde_json::to_value(active_environment.clone())?,
            )
            .await
        {
            tracing::warn!(
                "failed to update active environment for project {}: {}",
                self.id.to_string(),
                e
            )
        }
        Ok(())
    }

    pub async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        input: CreateEnvironmentInput,
    ) -> joinerror::Result<EnvironmentItemDescription> {
        let id = EnvironmentId::new();
        let environment_item = self
            .environment_service
            .create_environment(
                ctx,
                CreateEnvironmentItemParams {
                    env_id: id.clone(),
                    project_id: input.project_id.clone(),
                    name: input.name.clone(),
                    order: input.order.clone(),
                    color: input.color.clone(),
                    variables: input.variables.clone(),
                },
            )
            .await?;

        let builder = EnvironmentBuilder::new(
            self.workspace_id.inner(),
            self.fs.clone(),
            self.storage.clone(),
            id.clone(),
        );

        let handle = builder
            .create(
                ctx,
                CreateEnvironmentParams {
                    name: input.name.clone(),
                    color: input.color.clone(),
                    variables: input.variables,
                },
            )
            .await?;

        let environment = RuntimeEnvironment {
            id: environment_item.id.clone(),
            project_id: input.project_id.clone(),
            handle: handle.into(),
            edit: EnvironmentEditService::new(EnvironmentFsEditBackend::new(
                environment_item.internal_abs_path.as_ref(),
                self.fs.clone(),
            )),
        };

        let environments = self.environments_internal(ctx).await?;
        environments
            .write()
            .await
            .insert(environment_item.id.clone(), environment.clone());

        let desc = self
            .environment_service
            .describe_environment(ctx, &environment.id)
            .await?;

        Ok(EnvironmentItemDescription {
            id: desc.id.clone(),
            project_id: input.project_id.clone(),
            is_active: false,
            display_name: input.name.clone(),
            color: desc.color.clone(),
            abs_path: environment_item.internal_abs_path.into(),
            total_variables: desc.variables.len(),
        })
    }

    pub async fn delete_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()> {
        let environments = self.environments_internal(ctx).await?;

        let environment = if let Some(environment) = environments.write().await.remove(id) {
            environment
        } else {
            return Ok(());
        };

        drop(environment);

        // If the environment is currently active, reset the active environment
        let active_environment_updated = {
            let mut active_environment = self.active_environment.write().await;
            if active_environment.as_ref() == Some(id) {
                active_environment.take();
                true
            } else {
                false
            }
        };

        self.environment_service.delete_environment(ctx, id).await?;

        if active_environment_updated {
            let active_environment = self.active_environment.read().await;
            if let Err(e) = self
                .storage
                .put(
                    ctx,
                    StorageScope::Project(self.id.inner()),
                    KEY_ACTIVE_ENVIRONMENT,
                    serde_json::to_value(active_environment.to_owned())?,
                )
                .await
            {
                tracing::warn!("failed to update activeEnvironment in the database: {}", e);
            }
        }

        Ok(())
    }

    pub async fn update_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: UpdateEnvironmentParams,
    ) -> joinerror::Result<()> {
        let environment = self.environment(ctx, &params.id).await?;

        // We need to assign VariableId to newly created variables
        let vars_to_add = params
            .vars_to_add
            .iter()
            .map(|params| (VariableId::new(), params.to_owned()))
            .collect::<Vec<_>>();
        environment
            .edit
            .edit(
                ctx,
                EnvironmentEditParams {
                    name: params.name,
                    color: params.color,
                    vars_to_add: vars_to_add.clone(),
                    vars_to_update: params.vars_to_update.clone(),
                    vars_to_delete: params.vars_to_delete.clone(),
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to update environment {}", params.id))?;

        let storage_scope = StorageScope::Workspace(self.workspace_id.inner());

        // Again issue with the signature of put_batch, will try to fix it later
        for (var_id, var_to_add) in vars_to_add {
            if let Err(e) = self
                .storage
                .put(
                    ctx,
                    storage_scope.clone(),
                    &key_variable_local_value(&params.id, &var_id),
                    var_to_add.local_value,
                )
                .await
            {
                tracing::error!("failed to add variable local value to the database: {}", e);
            }
        }

        for var_to_update in params.vars_to_update {
            match var_to_update.local_value {
                Some(ChangeJsonValue::Update(value)) => {
                    if let Err(e) = self
                        .storage
                        .put(
                            ctx,
                            storage_scope.clone(),
                            &key_variable_local_value(&params.id, &var_to_update.id),
                            value,
                        )
                        .await
                    {
                        tracing::error!(
                            "failed to update variable local value in the database: {}",
                            e
                        );
                    }
                }
                Some(ChangeJsonValue::Remove) => {
                    if let Err(e) = self
                        .storage
                        .remove(
                            ctx,
                            storage_scope.clone(),
                            &key_variable_local_value(&params.id, &var_to_update.id),
                        )
                        .await
                    {
                        tracing::error!(
                            "failed to remove variable local value in the database: {}",
                            e
                        );
                    }
                }
                None => {}
            }
        }

        for id in params.vars_to_delete {
            if let Err(e) = self
                .storage
                .remove_batch_by_prefix(ctx, storage_scope.clone(), &key_variable(&params.id, &id))
                .await
            {
                tracing::error!("failed to remove variable data from the database: {}", e);
            }
        }

        Ok(())
    }
}
