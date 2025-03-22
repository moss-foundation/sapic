use anyhow::{Context, Result};
use arc_swap::ArcSwap;
use fnv::FnvHashMap;
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::{Any, TypeId},
    hash::BuildHasherDefault,
    sync::Arc,
};
use tauri::AppHandle;
use thiserror::Error;
use tokio::sync::{Mutex, OnceCell};

slotmap::new_key_type! {
    pub struct ServiceKey;
}

pub trait AppService_2: Any + Send + Sync {}

#[derive(Error, Debug)]
pub enum ServicePoolError {
    #[error("The service {0} must be registered before it can be used")]
    NotRegistered(String),

    #[error("Failed to get service")]
    Unknown(#[from] anyhow::Error),
}

type LazyServiceBuilder =
    Box<dyn Fn(&ServicePool, &AppHandle) -> Arc<dyn Any + Send + Sync> + Send + Sync>;
type ServiceSlot = Arc<dyn Any + Send + Sync>;

struct Opaque(());

pub struct ServicePool {
    app_handle: AppHandle,
    services: SlotMap<ServiceKey, ArcSwap<ServiceSlot>>,
    lazy_builders: SecondaryMap<ServiceKey, LazyServiceBuilder>,
    type_map: FnvHashMap<TypeId, ServiceKey>,
}

impl ServicePool {
    pub fn get_key_by_type<T>(&self) -> Result<ServiceKey, ServicePoolError>
    where
        T: AppService_2,
    {
        let type_id = TypeId::of::<T>();
        let key = self
            .type_map
            .get(&type_id)
            .ok_or(ServicePoolError::NotRegistered(
                std::any::type_name::<T>().to_string(),
            ))?;

        Ok(*key)
    }

    pub fn get_by_type<T>(&self) -> Result<&T, ServicePoolError>
    where
        T: AppService_2,
    {
        let type_id = TypeId::of::<T>();
        let key = self
            .type_map
            .get(&type_id)
            .ok_or(ServicePoolError::NotRegistered(
                std::any::type_name::<T>().to_string(),
            ))?;

        self.get_by_key::<T>(*key)
    }

    pub fn get_by_key<T>(&self, key: ServiceKey) -> Result<&T, ServicePoolError>
    where
        T: AppService_2,
    {
        let instance = self
            .services
            .get(key)
            .ok_or(ServicePoolError::NotRegistered(
                std::any::type_name::<T>().to_string(),
            ))?;

        let svc = instance.load_full();
        if svc.is::<Opaque>() {
            // SAFETY: This should never panic because we create the lazy builder when registering the service.
            let builder = unsafe { self.lazy_builders.get_unchecked(key) };

            let new_instance = builder(&self, &self.app_handle);
            instance.store(Arc::new(new_instance));
        }

        // SAFETY: ArcSwap guarantees stable and valid pointers for the lifetime of the pool.
        unsafe { Ok(&*(Arc::as_ptr(&svc) as *const T)) }
    }

    pub fn reset<T>(&self, key: ServiceKey) -> Result<&T, ServicePoolError>
    where
        T: AppService_2,
    {
        let instance = self
            .services
            .get(key)
            .ok_or(ServicePoolError::NotRegistered(
                std::any::type_name::<T>().to_string(),
            ))?;

        // SAFETY: This should never panic because we create the lazy builder when registering the service.
        let builder = unsafe { self.lazy_builders.get_unchecked(key) };

        let new_instance = builder(&self, &self.app_handle);
        instance.store(Arc::new(new_instance));

        let svc = instance.load_full();

        // SAFETY: ArcSwap guarantees stable and valid pointers for the lifetime of the pool.
        unsafe { Ok(&*(Arc::as_ptr(&svc) as *const T)) }
    }
}

pub enum Instantiation<S, F>
where
    F: Fn(&ServicePool, &AppHandle) -> S + Send + Sync + 'static,
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
            lazy_builders: SecondaryMap::with_capacity(capacity),
            type_map: FnvHashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
        })
    }

    pub fn register<S, F>(&mut self, builder: Instantiation<S, F>) -> ServiceKey
    where
        S: AppService_2,
        F: Fn(&ServicePool, &AppHandle) -> S + Send + Sync + 'static,
    {
        match builder {
            Instantiation::Instant(builder) => self.register_instant(builder),
            Instantiation::Lazy(builder) => self.register_lazy(builder),
        }
    }

    fn register_instant<S, F>(&mut self, builder: F) -> ServiceKey
    where
        S: AppService_2,
        F: Fn(&ServicePool, &AppHandle) -> S + Send + Sync + 'static,
    {
        let slot: Arc<dyn Any + Send + Sync> = Arc::new(builder(&self.0, &self.0.app_handle));
        let key = self.0.services.insert(ArcSwap::new(Arc::new(slot)));

        self.register_internal(key, builder)
    }

    fn register_lazy<S, F>(&mut self, builder: F) -> ServiceKey
    where
        S: AppService_2,
        F: Fn(&ServicePool, &AppHandle) -> S + Send + Sync + 'static,
    {
        let slot = Arc::new(Opaque(()));
        let key = self.0.services.insert(ArcSwap::new(Arc::new(slot)));

        self.register_internal(key, builder)
    }

    fn register_internal<S, F>(&mut self, key: ServiceKey, builder: F) -> ServiceKey
    where
        S: AppService_2,
        F: Fn(&ServicePool, &AppHandle) -> S + Send + Sync + 'static,
    {
        self.0.lazy_builders.insert(
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
