use futures::Stream;
use joinerror::ResultExt;
use moss_applib::{AppRuntime, ServiceMarker};
use moss_bindingutils::primitives::ChangeString;
use moss_environment::{
    AnyEnvironment, Environment, ModifyEnvironmentParams,
    builder::{EnvironmentBuilder, EnvironmentLoadParams},
    models::{
        primitives::{EnvironmentId, VariableId},
        types::{AddVariableParams, UpdateVariableParams},
    },
};
use moss_fs::FileSystem;
use moss_workspacelib::{EnvironmentRegistry, environment_registry::EnvironmentModel};
use std::{
    collections::HashMap,
    marker::PhantomData,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{dirs, errors::ErrorNotFound, models::primitives::CollectionId};

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

#[derive(Clone)]
pub struct EnvironmentItem {
    pub id: EnvironmentId,
    pub display_name: String,
    pub order: isize,
    pub expanded: bool,
}

pub struct EnvironmentItemDescription {
    pub id: EnvironmentId,
    pub collection_id: Option<CollectionId>,
    pub display_name: String,
    pub order: isize,
    pub expanded: bool,
    pub color: Option<String>,
    pub abs_path: PathBuf,
}

struct ServiceState {
    environments: HashMap<EnvironmentId, EnvironmentItem>,
}

pub struct EnvironmentService<R>
where
    R: AppRuntime,
{
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    environment_registry: Arc<EnvironmentRegistry<R, Environment<R>>>,
    state: Arc<RwLock<ServiceState>>,
}

impl<R> ServiceMarker for EnvironmentService<R> where R: AppRuntime {}

impl<R> EnvironmentService<R>
where
    R: AppRuntime,
{
    /// `abs_path` is the absolute path to the workspace directory
    pub async fn new(abs_path: &Path, fs: Arc<dyn FileSystem>) -> joinerror::Result<Self> {
        let abs_path = abs_path.join(dirs::ENVIRONMENTS_DIR);
        let environment_registry = EnvironmentRegistry::new();
        let environments = collect_environments(&fs, &environment_registry, &abs_path).await?;

        Ok(Self {
            fs,
            abs_path,
            environment_registry: Arc::new(environment_registry),
            state: Arc::new(RwLock::new(ServiceState { environments })),
        })
    }

    pub async fn environment(&self, id: &EnvironmentId) -> Option<Arc<Environment<R>>> {
        self.environment_registry
            .get(id)
            .await
            .map(|model| model.handle.clone())
    }

    pub async fn list_environments(
        &self,
        _ctx: &R::AsyncContext,
    ) -> Pin<Box<dyn Stream<Item = EnvironmentItemDescription> + Send + '_>> {
        let state = self.state.clone();

        Box::pin(async_stream::stream! {
            let state_lock = state.read().await;
            for (_, item) in state_lock.environments.iter() {
                if let Some(model) = self.environment_registry.get(&item.id).await {
                    let abs_path = model.abs_path().await;

                    yield EnvironmentItemDescription {
                        id: item.id.clone(),
                        collection_id: model.collection_id.map(Into::into),
                        display_name: item.display_name.clone(),
                        order: item.order,
                        expanded: item.expanded,
                        color: None, // TODO: hardcoded for now
                        abs_path,
                    };
                } else {
                    // TODO: log error
                    println!("environment model not found: {}", item.id);
                }
            }
        })
    }

    pub async fn update_environment(
        &self,
        _ctx: &R::AsyncContext,
        id: &EnvironmentId,
        params: UpdateEnvironmentItemParams,
    ) -> joinerror::Result<()> {
        let environment_model = self.environment_registry.get(id).await.ok_or_else(|| {
            joinerror::Error::new::<ErrorNotFound>(format!("environment model not found: {}", id))
        })?;

        let mut state = self.state.write().await;
        let environment_item = state.environments.get_mut(id).ok_or_else(|| {
            joinerror::Error::new::<ErrorNotFound>(format!("environment item not found: {}", id))
        })?;

        environment_model
            .modify(ModifyEnvironmentParams {
                name: params.name.clone(),
                color: params.color,
                vars_to_add: params.vars_to_add,
                vars_to_update: params.vars_to_update,
                vars_to_delete: params.vars_to_delete,
            })
            .await?;

        if let Some(name) = params.name {
            environment_item.display_name = name;
        }
        if let Some(order) = params.order {
            environment_item.order = order;
        }
        if let Some(expanded) = params.expanded {
            environment_item.expanded = expanded;
        }

        Ok(())
    }

    pub async fn create_environment(
        &self,
        _ctx: &R::AsyncContext,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<EnvironmentItemDescription> {
        let environment = EnvironmentBuilder::new(self.fs.clone())
            .create::<R>(moss_environment::builder::CreateEnvironmentParams {
                name: params.name.clone(),
                abs_path: &self.abs_path,
                color: params.color,
                order: params.order,
            })
            .await?;

        let abs_path = environment.abs_path().await;
        let desc = environment.describe().await?;

        self.environment_registry
            .insert(EnvironmentModel {
                id: desc.id.clone(),
                collection_id: params.collection_id.clone().map(|id| id.inner()),
                handle: Arc::new(environment),
                _runtime: PhantomData,
            })
            .await;

        let mut state = self.state.write().await;
        state.environments.insert(
            desc.id.clone(),
            EnvironmentItem {
                id: desc.id.clone(),
                display_name: params.name.clone(),
                order: params.order,
                expanded: true,
            },
        );

        // TODO: put environment order to the database

        Ok(EnvironmentItemDescription {
            id: desc.id.clone(),
            collection_id: params.collection_id,
            display_name: params.name.clone(),
            order: params.order,
            expanded: true,
            color: desc.color,
            abs_path,
        })
    }
}

async fn collect_environments<R: AppRuntime>(
    fs: &Arc<dyn FileSystem>,

    environment_registry: &EnvironmentRegistry<R, Environment<R>>,
    abs_path: &Path,
) -> joinerror::Result<HashMap<EnvironmentId, EnvironmentItem>> {
    let mut environments = HashMap::new();

    let mut read_dir = fs
        .read_dir(abs_path)
        .await
        .map_err(|err| joinerror::Error::new::<()>(format!("failed to read directory: {}", err)))?; // TODO: specify a proper error type

    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            continue;
        }

        let environment = EnvironmentBuilder::new(fs.clone())
            .load::<R>(EnvironmentLoadParams {
                abs_path: entry.path(),
            })
            .await
            .join_err_with::<()>(|| {
                format!("failed to load environment: {}", entry.path().display())
            })?;

        let desc = environment.describe().await?;

        environment_registry
            .insert(EnvironmentModel {
                id: desc.id.clone(),
                collection_id: None,
                handle: Arc::new(environment),
                _runtime: PhantomData,
            })
            .await;

        environments.insert(
            desc.id.clone(),
            EnvironmentItem {
                id: desc.id,
                display_name: desc.name,
                order: 0,       // TODO: restore from the database
                expanded: true, // TODO: restore from the database
            },
        );
    }

    Ok(environments)
}
