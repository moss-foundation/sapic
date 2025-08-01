use futures::Stream;
use moss_applib::{AppRuntime, ServiceMarker};
use moss_environment::{
    AnyEnvironment, Environment, ModifyEnvironmentParams,
    builder::EnvironmentBuilder,
    models::primitives::EnvironmentId,
    registry::{EnvironmentModel, GlobalEnvironmentRegistry},
};
use moss_fs::{FileSystem, model_registry::GlobalModelRegistry};
use moss_text::sanitized::sanitize;
use std::{collections::HashMap, marker::PhantomData, pin::Pin, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    errors::ErrorNotFound,
    models::types::{CreateEnvironmentItemParams, UpdateEnvironmentItemParams},
};

pub struct EnvironmentItem {
    pub id: EnvironmentId,
    pub display_name: String,
    pub order: isize,
    pub expanded: bool,
}

struct ServiceState {
    environments: HashMap<EnvironmentId, EnvironmentItem>,
}

pub struct EnvironmentService<R>
where
    R: AppRuntime,
{
    fs: Arc<dyn FileSystem>,
    environment_registry: GlobalEnvironmentRegistry<R, Environment<R>>,
    model_registry: GlobalModelRegistry,
    state: Arc<RwLock<ServiceState>>,
    _marker: PhantomData<R>,
}

impl<R> ServiceMarker for EnvironmentService<R> where R: AppRuntime {}

impl<R> EnvironmentService<R>
where
    R: AppRuntime,
{
    pub fn new(
        fs: Arc<dyn FileSystem>,
        environment_registry: GlobalEnvironmentRegistry<R, Environment<R>>,
        model_registry: GlobalModelRegistry,
    ) -> Self {
        Self {
            fs,
            environment_registry,
            model_registry,
            state: Arc::new(RwLock::new(ServiceState {
                environments: HashMap::new(),
            })),
            _marker: PhantomData,
        }
    }

    pub async fn list_environments(
        &self,
    ) -> Pin<Box<dyn Stream<Item = EnvironmentItem> + Send + '_>> {
        let state = self.state.clone();

        // Box::pin(async_stream::stream! {
        //     let state_lock = state.read().await;
        //     for (id, item) in state_lock.environments.iter() {

        //         yield item.clone();
        //     }
        // });

        todo!()
    }

    pub async fn update_environment(
        &self,
        params: UpdateEnvironmentItemParams,
    ) -> joinerror::Result<()> {
        let environment_model =
            self.environment_registry
                .get(&params.id)
                .await
                .ok_or_else(|| {
                    joinerror::Error::new::<ErrorNotFound>(format!(
                        "environment model not found: {}",
                        params.id
                    ))
                })?;

        let mut state = self.state.write().await;
        let environment_item = state.environments.get_mut(&params.id).ok_or_else(|| {
            joinerror::Error::new::<ErrorNotFound>(format!(
                "environment item not found: {}",
                params.id
            ))
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

        Ok(())
    }

    pub async fn create_environment(
        &self,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<()> {
        let sanitized_name = sanitize(&params.name);
        let environment = EnvironmentBuilder::new(self.fs.clone(), self.model_registry.clone())
            .create(moss_environment::builder::CreateEnvironmentParams {
                name: sanitized_name.clone(),
                abs_path: params.abs_path,
                color: params.color,
            })
            .await?;

        let id = EnvironmentId::new();
        self.environment_registry
            .insert(EnvironmentModel {
                id: id.clone(),
                handle: Arc::new(environment),
                _runtime: PhantomData,
            })
            .await;

        let mut state = self.state.write().await;
        state.environments.insert(
            id.clone(),
            EnvironmentItem {
                id,
                display_name: params.name,
                order: params.order,
                expanded: false, // TODO: hardcoded for now
            },
        );

        // TODO: put environment order to the database

        Ok(())
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
