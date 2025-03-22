use anyhow::{anyhow, Context, Result};
use fnv::FnvHashMap;
use moss_tauri::TauriError;
use parking_lot::Mutex;
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::{Any, TypeId},
    hash::BuildHasherDefault,
    sync::Arc,
};
use tauri::AppHandle;
use thiserror::Error;
use tokio::sync::OnceCell;

slotmap::new_key_type! {
    pub struct ServiceKey;
}

pub trait AppService: Any + Send + Sync {}

#[derive(Error, Debug)]
pub enum ServicePoolError {
    #[error("The service {0} must be registered before it can be used")]
    NotRegistered(String),

    #[error("The service {0} was already initialized")]
    AlreadyInitialized(String),

    #[error("Type mismatch")]
    TypeMismatch,

    #[error("Failed to get service")]
    Unknown(#[from] anyhow::Error),
}

impl From<ServicePoolError> for TauriError {
    fn from(error: ServicePoolError) -> Self {
        TauriError(error.to_string())
    }
}

type AnyService = Arc<dyn Any + Send + Sync>;
type LazyServiceBuilder = Box<dyn FnOnce(&ServicePool, &AppHandle) -> AnyService + Send + Sync>;

pub struct ServicePool {
    app_handle: AppHandle,
    services: SlotMap<ServiceKey, OnceCell<AnyService>>,
    lazy_builders: Mutex<SecondaryMap<ServiceKey, LazyServiceBuilder>>,
    type_map: FnvHashMap<TypeId, ServiceKey>,
}

impl ServicePool {
    pub async fn get_by_type<T>(&self) -> Result<&T, ServicePoolError>
    where
        T: AppService,
    {
        let type_id = TypeId::of::<T>();
        let key = self
            .type_map
            .get(&type_id)
            .ok_or(ServicePoolError::NotRegistered(
                std::any::type_name::<T>().to_string(),
            ))?;

        self.get_by_key::<T>(*key).await
    }

    pub async fn get_by_key<T>(&self, key: ServiceKey) -> Result<&T, ServicePoolError>
    where
        T: AppService,
    {
        let cell = self.services.get(key).context("dd")?;
        let any = cell
            .get_or_try_init(|| async move {
                let mut lazy_builders_lock = self.lazy_builders.lock();
                let builder =
                    lazy_builders_lock
                        .remove(key)
                        .ok_or(ServicePoolError::AlreadyInitialized(
                            std::any::type_name::<T>().to_string(),
                        ))?;

                Ok::<_, ServicePoolError>(builder(&self, &self.app_handle))
            })
            .await?;

        any.downcast_ref::<T>()
            .ok_or(ServicePoolError::TypeMismatch)
    }
}

pub enum Instantiation<S, F>
where
    S: AppService + 'static,
    F: FnOnce(&ServicePool, &AppHandle) -> S + Send + Sync + 'static,
{
    Instant(F),
    Lazy(F),
}
pub struct ServicePoolBuilder(ServicePool);

impl ServicePoolBuilder {
    pub fn new(app_handle: AppHandle, capacity: usize) -> Self {
        Self(ServicePool {
            app_handle,
            services: SlotMap::with_capacity_and_key(capacity),
            lazy_builders: Mutex::new(SecondaryMap::with_capacity(capacity)),
            type_map: FnvHashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
        })
    }

    pub fn register<S, F>(&mut self, builder: Instantiation<S, F>) -> ServiceKey
    where
        S: AppService + 'static,
        F: FnOnce(&ServicePool, &AppHandle) -> S + Send + Sync + 'static,
    {
        match builder {
            Instantiation::Instant(builder) => self.register_instant(builder),
            Instantiation::Lazy(builder) => self.register_lazy(builder),
        }
    }

    fn register_instant<S, F>(&mut self, builder: F) -> ServiceKey
    where
        S: AppService + 'static,
        F: FnOnce(&ServicePool, &AppHandle) -> S + Send + Sync + 'static,
    {
        let service: Arc<dyn Any + Send + Sync + 'static> =
            Arc::new(builder(&self.0, &self.0.app_handle));
        let cell = OnceCell::from(service);
        let key = self.0.services.insert(cell);

        let type_id = TypeId::of::<S>();
        self.0.type_map.insert(type_id, key);

        key
    }

    fn register_lazy<S, F>(&mut self, builder: F) -> ServiceKey
    where
        S: AppService + 'static,
        F: FnOnce(&ServicePool, &AppHandle) -> S + Send + Sync + 'static,
    {
        let cell = OnceCell::new();
        let key = self.0.services.insert(cell);

        let mut lazy_builders_lock = self.0.lazy_builders.lock();
        lazy_builders_lock.insert(
            key,
            Box::new(move |pool, app_handle| Arc::new(builder(pool, app_handle))),
        );

        let type_id = TypeId::of::<S>();
        self.0.type_map.insert(type_id, key);

        key
    }

    pub fn build(self) -> ServicePool {
        self.0
    }
}
