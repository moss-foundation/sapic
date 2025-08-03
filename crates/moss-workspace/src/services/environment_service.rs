use futures::Stream;
use moss_applib::{AppRuntime, ServiceMarker};
use moss_bindingutils::primitives::ChangeString;
use moss_environment::{
    AnyEnvironment, Environment, ModifyEnvironmentParams,
    builder::EnvironmentBuilder,
    models::{
        primitives::{EnvironmentId, VariableId},
        types::{AddVariableParams, UpdateVariableParams},
    },
    registry::{EnvironmentModel, GlobalEnvironmentRegistry},
    services::{
        metadata_service::MetadataService, sync_service::SyncService,
        variable_service::VariableService,
    },
};
use moss_fs::{FileSystem, model_registry::GlobalModelRegistry};
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
    pub abs_path: Arc<Path>,
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
    environment_registry: Arc<GlobalEnvironmentRegistry<R, Environment<R>>>,
    model_registry: Arc<GlobalModelRegistry>,
    state: Arc<RwLock<ServiceState>>,
}

impl<R> ServiceMarker for EnvironmentService<R> where R: AppRuntime {}

impl<R> EnvironmentService<R>
where
    R: AppRuntime,
{
    /// `abs_path` is the absolute path to the workspace directory
    pub fn new(
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        environment_registry: Arc<GlobalEnvironmentRegistry<R, Environment<R>>>,
        model_registry: Arc<GlobalModelRegistry>,
    ) -> Self {
        Self {
            fs,
            abs_path: abs_path.join(dirs::ENVIRONMENTS_DIR),
            environment_registry,
            model_registry,
            state: Arc::new(RwLock::new(ServiceState {
                environments: HashMap::new(),
            })),
        }
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
        let id = EnvironmentId::new();
        let metadata_service = MetadataService::new();
        let sync_service = Arc::new(SyncService::new(
            self.model_registry.clone(),
            self.fs.clone(),
        ));
        // TODO: env storage service
        let variable_service = VariableService::<R>::new(
            None, // FIXME: hardcoded for now
            sync_service.clone(),
        )?;

        let environment = EnvironmentBuilder::new(self.fs.clone())
            .with_service(metadata_service)
            .with_service::<SyncService>(sync_service)
            .with_service::<VariableService<R>>(variable_service)
            .create::<R>(
                self.model_registry.clone(),
                moss_environment::builder::CreateEnvironmentParams {
                    id: id.clone(),
                    name: params.name.clone(),
                    abs_path: &self.abs_path,
                    color: params.color,
                },
            )
            .await?;

        let abs_path = environment.abs_path().await;
        let color = environment.color().await;

        self.environment_registry
            .insert(EnvironmentModel {
                id: id.clone(),
                collection_id: params.collection_id.clone().map(|id| id.inner()),
                handle: Arc::new(environment),
                _runtime: PhantomData,
            })
            .await;

        let mut state = self.state.write().await;
        state.environments.insert(
            id.clone(),
            EnvironmentItem {
                id: id.clone(),
                display_name: params.name.clone(),
                order: params.order,
                expanded: true,
            },
        );

        // TODO: put environment order to the database

        Ok(EnvironmentItemDescription {
            id,
            collection_id: params.collection_id,
            display_name: params.name.clone(),
            order: params.order,
            expanded: true,
            color,
            abs_path,
        })
    }
}

// pub async fn environments<C: Context<R>>(&self, ctx: &C) -> Result<&EnvironmentMap> {
//     let fs = <dyn FileSystem>::global::<R, C>(ctx);
//     let result = self
//         .environments
//         .get_or_try_init(|| async move {
//             let mut environments = HashMap::new();

//             let abs_path = self.abs_path.join(dirs::ENVIRONMENTS_DIR);
//             if !abs_path.exists() {
//                 return Ok(environments);
//             }

//             // TODO: restore environments cache from the database
//             let mut read_dir = fs.read_dir(&abs_path).await?;
//             while let Some(entry) = read_dir.next_entry().await? {
//                 if entry.file_type().await?.is_dir() {
//                     continue;
//                 }

//                 let entry_abs_path = entry.path();
//                 let name = entry_abs_path
//                     .file_name()
//                     .unwrap()
//                     .to_string_lossy()
//                     .to_string();
//                 let decoded_name = desanitize(&name);

//                 let environment = Environment::load(
//                     &entry_abs_path,
//                     fs.clone(),
//                     self.storage.variable_store().clone(),
//                     self.next_variable_id.clone(),
//                     environment::LoadParams {
//                         create_if_not_exists: false,
//                     },
//                 )
//                 .await?;

//                 let id = environment.id().await;
//                 let entry = EnvironmentItem {
//                     id,
//                     name,
//                     display_name: decoded_name,
//                     inner: environment,
//                 };

//                 environments.insert(id, Arc::new(entry));
//             }

//             Ok::<_, anyhow::Error>(environments)
//         })
//         .await?;

//     Ok(result)
// }
