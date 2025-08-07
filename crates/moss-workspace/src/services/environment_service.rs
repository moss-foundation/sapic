use derive_more::Deref;
use futures::Stream;
use joinerror::{OptionExt, ResultExt};
use moss_applib::{AppRuntime, ServiceMarker};
use moss_bindingutils::primitives::ChangeString;
use moss_environment::{
    AnyEnvironment, Environment, ModifyEnvironmentParams,
    builder::{EnvironmentBuilder, EnvironmentLoadParams},
    errors::ErrorIo,
    models::{
        primitives::{EnvironmentId, VariableId},
        types::{AddVariableParams, UpdateVariableParams},
    },
    segments::{SEGKEY_VARIABLE_LOCALVALUE, SEGKEY_VARIABLE_ORDER},
};
use moss_fs::{FileSystem, FsResultExt, RemoveOptions};
use moss_storage::storage::operations::RemoveItem;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{
    dirs, errors::ErrorNotFound, models::primitives::CollectionId,
    services::storage_service::StorageService,
};

pub struct CreateEnvironmentItemParams {
    pub collection_id: Option<CollectionId>,
    pub name: String,
    pub order: isize,
    pub color: Option<String>,
}

pub struct UpdateEnvironmentItemParams {
    pub name: Option<String>,
    pub expanded: Option<bool>,
    pub order: Option<isize>,
    pub color: Option<ChangeString>,
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}

#[derive(Clone, Deref)]
struct EnvironmentItem<R>
where
    R: AppRuntime,
{
    pub id: EnvironmentId,
    pub color: Option<String>,
    pub collection_id: Option<CollectionId>,
    pub display_name: String,
    pub order: Option<isize>,
    pub expanded: bool,

    #[deref]
    pub handle: Arc<Environment<R>>,
}

pub struct EnvironmentItemDescription {
    pub id: EnvironmentId,
    pub collection_id: Option<CollectionId>,
    pub display_name: String,
    pub order: Option<isize>,
    pub expanded: bool,
    pub color: Option<String>,
    pub abs_path: PathBuf,
}

type EnvironmentMap<R> = HashMap<EnvironmentId, EnvironmentItem<R>>;

struct ServiceState<R>
where
    R: AppRuntime,
{
    environments: EnvironmentMap<R>,
}

pub struct EnvironmentService<R>
where
    R: AppRuntime,
{
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    state: Arc<RwLock<ServiceState<R>>>,
}

impl<R> ServiceMarker for EnvironmentService<R> where R: AppRuntime {}

impl<R> EnvironmentService<R>
where
    R: AppRuntime,
{
    /// `abs_path` is the absolute path to the workspace directory
    pub async fn new(
        ctx: &R::AsyncContext,
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        storage_service: Arc<StorageService<R>>,
    ) -> joinerror::Result<Self> {
        let abs_path = abs_path.join(dirs::ENVIRONMENTS_DIR);
        let environments =
            collect_environments(ctx, &fs, &abs_path, storage_service.clone()).await?;

        Ok(Self {
            fs,
            abs_path,
            state: Arc::new(RwLock::new(ServiceState { environments })),
        })
    }

    pub async fn environment(&self, id: &EnvironmentId) -> Option<Arc<Environment<R>>> {
        let state = self.state.read().await;
        state.environments.get(id).map(|item| item.handle.clone())
    }

    pub async fn list_environments(
        &self,
        _ctx: &R::AsyncContext,
    ) -> Pin<Box<dyn Stream<Item = EnvironmentItemDescription> + Send + '_>> {
        let state = self.state.clone();

        Box::pin(async_stream::stream! {
            let state_lock = state.read().await;
            for (_, item) in state_lock.environments.iter() {
                yield EnvironmentItemDescription {
                    id: item.id.clone(),
                    collection_id: item.collection_id.clone(),
                    display_name: item.display_name.clone(),
                    order: item.order,
                    expanded: item.expanded,
                    color: item.color.clone(),
                    abs_path: item.abs_path().await,
                };
            }
        })
    }

    pub async fn update_environment(
        &self,
        ctx: &R::AsyncContext,
        id: &EnvironmentId,
        params: UpdateEnvironmentItemParams,
        storage_service: Arc<StorageService<R>>,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;
        let environment_item = state
            .environments
            .get_mut(id)
            .ok_or_join_err_with::<ErrorNotFound>(|| {
                format!("environment item not found: {}", id)
            })?;

        environment_item
            .modify(
                ctx,
                ModifyEnvironmentParams {
                    name: params.name.clone(),
                    color: params.color.clone(),
                    vars_to_add: params.vars_to_add,
                    vars_to_update: params.vars_to_update,
                    vars_to_delete: params.vars_to_delete,
                },
            )
            .await?;

        if let Some(name) = params.name {
            environment_item.display_name = name;
        }

        match params.color {
            Some(ChangeString::Update(color)) => {
                environment_item.color = Some(color);
            }
            Some(ChangeString::Remove) => {
                environment_item.color = None;
            }
            None => {}
        }

        if let Some(order) = params.order {
            environment_item.order = Some(order);
            if let Err(e) = storage_service.put_environment_order(ctx, id, order).await {
                // TODO: log error
                println!("failed to put environment order in the db: {}", e);
            }
        }

        if let Some(expanded) = params.expanded {
            environment_item.expanded = expanded;
            let mut expanded_environments = storage_service
                .get_expanded_environments(ctx)
                .await
                .unwrap_or_default();

            if expanded {
                expanded_environments.insert(id.clone());
            } else {
                expanded_environments.remove(&id);
            }

            if let Err(e) = storage_service
                .put_expanded_environments(ctx, &expanded_environments)
                .await
            {
                // TODO: log error
                println!("failed to put expandedEnvironments in the db: {}", e);
            }
        }

        Ok(())
    }

    pub async fn create_environment(
        &self,
        ctx: &R::AsyncContext,
        params: CreateEnvironmentItemParams,
        storage_service: Arc<StorageService<R>>,
    ) -> joinerror::Result<EnvironmentItemDescription> {
        let environment = EnvironmentBuilder::new(self.fs.clone())
            .create::<R>(
                moss_environment::builder::CreateEnvironmentParams {
                    name: params.name.clone(),
                    abs_path: &self.abs_path,
                    color: params.color,
                    order: params.order,
                },
                storage_service.variable_store(),
            )
            .await?;

        let abs_path = environment.abs_path().await;
        let desc = environment.describe(ctx).await?;

        let mut state = self.state.write().await;
        state.environments.insert(
            desc.id.clone(),
            EnvironmentItem {
                id: desc.id.clone(),
                color: desc.color.clone(),
                collection_id: params.collection_id.clone(),
                display_name: params.name.clone(),
                order: Some(params.order),
                expanded: true,
                handle: Arc::new(environment),
            },
        );

        if let Err(e) = storage_service
            .put_environment_order(ctx, &desc.id, params.order)
            .await
        {
            // TODO: log error
            println!("failed to put environment order in the db: {}", e);
        }

        let mut expanded_environments = storage_service
            .get_expanded_environments(ctx)
            .await
            .unwrap_or_default();

        expanded_environments.insert(desc.id.clone());

        if let Err(e) = storage_service
            .put_expanded_environments(ctx, &expanded_environments)
            .await
        {
            println!("failed to put expandedEnvironments in the db: {}", e);
        }
        Ok(EnvironmentItemDescription {
            id: desc.id.clone(),
            collection_id: params.collection_id,
            display_name: params.name.clone(),
            order: Some(params.order),
            expanded: true,
            color: desc.color,
            abs_path,
        })
    }

    pub async fn delete_environment(
        &self,
        ctx: &R::AsyncContext,
        id: &EnvironmentId,
        storage_service: Arc<StorageService<R>>,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;
        if let Some(environment) = state.environments.remove(id) {
            let abs_path = environment.abs_path().await;
            let desc = environment.describe(ctx).await?;
            self.fs
                .remove_file(
                    &abs_path,
                    RemoveOptions {
                        recursive: false,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .join_err_with::<ErrorIo>(|| {
                    format!(
                        "failed to remove environment file at {}",
                        abs_path.display()
                    )
                })?;

            // Remove environment from the database
            let mut expanded_environments = storage_service
                .get_expanded_environments(ctx)
                .await
                .unwrap_or_default();

            expanded_environments.remove(&id);

            if let Err(e) = storage_service
                .put_expanded_environments(ctx, &expanded_environments)
                .await
            {
                println!("failed to put expandedEnvironments in the db: {}", e);
            }

            if let Err(e) = storage_service.remove_environment_order(ctx, id).await {
                // TODO: log error
                println!("failed to remove environment order in the db: {}", e);
            }

            // Remove all variables belonging to the deleted environment
            let store = storage_service.variable_store();
            for id in desc.variables.keys() {
                let segkey_localvalue = SEGKEY_VARIABLE_LOCALVALUE.join(id.as_str());

                if let Err(e) = RemoveItem::remove(store.as_ref(), ctx, segkey_localvalue).await {
                    // TODO: log error
                    println!("failed to remove variable local value in the db: {}", e);
                }

                let segkey_order = SEGKEY_VARIABLE_ORDER.join(id.as_str());
                if let Err(e) = RemoveItem::remove(store.as_ref(), ctx, segkey_order).await {
                    // TODO: log error
                    println!("failed to remove variable order in the db: {}", e);
                }
            }

            Ok(())
        } else {
            // FIXME: Should deleting a non-existent environment be an error?
            Err(joinerror::Error::new::<ErrorNotFound>(format!(
                "environment {} not found",
                id
            )))
        }
    }
}

async fn collect_environments<R: AppRuntime>(
    ctx: &R::AsyncContext,
    fs: &Arc<dyn FileSystem>,
    abs_path: &Path,
    storage_service: Arc<StorageService<R>>,
) -> joinerror::Result<EnvironmentMap<R>> {
    let mut environments = EnvironmentMap::new();

    let mut read_dir = fs
        .read_dir(abs_path)
        .await
        .map_err(|err| joinerror::Error::new::<()>(format!("failed to read directory: {}", err)))?; // TODO: specify a proper error type

    let expanded_environments = storage_service
        .get_expanded_environments(ctx)
        .await
        .unwrap_or_default();

    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            continue;
        }

        let environment = EnvironmentBuilder::new(fs.clone())
            .load::<R>(
                EnvironmentLoadParams {
                    abs_path: entry.path(),
                },
                storage_service.variable_store(),
            )
            .await
            .join_err_with::<()>(|| {
                format!("failed to load environment: {}", entry.path().display())
            })?;

        let desc = environment.describe(ctx).await?;

        // TODO: log error
        let order = storage_service
            .get_environment_order(ctx, &desc.id)
            .await
            .ok();
        let expanded = expanded_environments.contains(&desc.id);

        environments.insert(
            desc.id.clone(),
            EnvironmentItem {
                id: desc.id,
                color: desc.color,
                // This is for restoring environments within the workspace scope,
                // these workspaces don't have this parameter.
                collection_id: None,
                display_name: desc.name,
                order,
                expanded,
                handle: Arc::new(environment),
            },
        );
    }

    Ok(environments)
}
