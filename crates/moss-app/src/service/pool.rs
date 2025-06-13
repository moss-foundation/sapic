use anyhow::{Context, Result};
use fnv::FnvHashMap;
use parking_lot::Mutex;
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
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

type AnyService = Arc<dyn Any + Send + Sync>;
type LazyServiceBuilder<R> =
    Box<dyn FnOnce(&ServicePool<R>, &AppHandle<R>) -> AnyService + Send + Sync>;

pub struct ServicePool<R: TauriRuntime> {
    pub(super) services: SlotMap<ServiceKey, OnceCell<AnyService>>,
    pub(super) lazy_builders: Mutex<SecondaryMap<ServiceKey, LazyServiceBuilder<R>>>,
    pub(super) type_map: FnvHashMap<TypeId, ServiceKey>,
}

impl<R: TauriRuntime> ServicePool<R> {
    pub async fn get_by_type<T>(&self, app_handle: &AppHandle<R>) -> Result<&T, ServicePoolError>
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

        self.get_by_key::<T>(*key, app_handle).await
    }

    pub async fn get_by_key<T>(
        &self,
        key: ServiceKey,
        app_handle: &AppHandle<R>,
    ) -> Result<&T, ServicePoolError>
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

                Ok::<_, ServicePoolError>(builder(&self, &app_handle))
            })
            .await?;

        any.downcast_ref::<T>()
            .ok_or(ServicePoolError::TypeMismatch)
    }
}
