use fnv::FnvHashMap;
use parking_lot::Mutex;
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::OnceCell;

use super::pool::{AppService, ServiceKey, ServicePool};

pub enum Instantiation<R, S, F>
where
    R: TauriRuntime,
    S: AppService + 'static,
    F: FnOnce(&ServicePool<R>, &AppHandle<R>) -> S + Send + Sync + 'static,
{
    Instant(F, PhantomData<(R, S)>),
    Lazy(F, PhantomData<(R, S)>),
}

pub struct ServicePoolBuilder<R: TauriRuntime>(ServicePool<R>);

impl<R: TauriRuntime> ServicePoolBuilder<R> {
    pub fn new() -> Self {
        Self(ServicePool::<R> {
            services: SlotMap::with_key(),
            lazy_builders: Mutex::new(SecondaryMap::new()),
            type_map: FnvHashMap::default(),
        })
    }

    pub fn register<S, F>(
        &mut self,
        builder: Instantiation<R, S, F>,
        app_handle: &AppHandle<R>,
    ) -> ServiceKey
    where
        R: TauriRuntime,
        S: AppService + 'static,
        F: FnOnce(&ServicePool<R>, &AppHandle<R>) -> S + Send + Sync + 'static,
    {
        match builder {
            Instantiation::Instant(builder, _) => self.register_instant(builder, app_handle),
            Instantiation::Lazy(builder, _) => self.register_lazy(builder),
        }
    }

    fn register_instant<S, F>(&mut self, builder: F, app_handle: &AppHandle<R>) -> ServiceKey
    where
        R: TauriRuntime,
        S: AppService + 'static,
        F: FnOnce(&ServicePool<R>, &AppHandle<R>) -> S + Send + Sync + 'static,
    {
        let service: Arc<dyn Any + Send + Sync + 'static> = Arc::new(builder(&self.0, app_handle));
        let cell = OnceCell::from(service);
        let key = self.0.services.insert(cell);

        let type_id = TypeId::of::<S>();
        self.0.type_map.insert(type_id, key);

        key
    }

    fn register_lazy<S, F>(&mut self, builder: F) -> ServiceKey
    where
        R: TauriRuntime,
        S: AppService + 'static,
        F: FnOnce(&ServicePool<R>, &AppHandle<R>) -> S + Send + Sync + 'static,
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

    pub fn build(self) -> ServicePool<R> {
        self.0
    }
}
