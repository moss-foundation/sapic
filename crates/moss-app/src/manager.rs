use anyhow::Result;
use std::sync::Arc;
use tauri::AppHandle;

use crate::service_pool::{AppService, ServicePool};

pub struct AppInner {
    service_pool: ServicePool,
}

pub struct App {
    app_handle: AppHandle,
    inner: Arc<AppInner>,
}

impl Clone for App {
    fn clone(&self) -> Self {
        Self {
            app_handle: self.app_handle.clone(),
            inner: Arc::clone(&self.inner),
        }
    }
}

pub struct AppManager {
    service_pool: ServicePool,
    // TODO: Registry
}

unsafe impl Send for AppManager {}
unsafe impl Sync for AppManager {}

impl AppManager {
    pub fn new(service_pool: ServicePool) -> Self {
        Self { service_pool }
    }

    pub fn services(&self) -> &ServicePool {
        &self.service_pool
    }

    // pub fn with_service<T, F>(self, service: F, activation_type: InstantiationType) -> Self
    // where
    //     T: AppService + 'static,
    //     F: FnOnce(&AppHandle) -> T + 'static,
    // {
    //     self.services.register(service, activation_type);
    //     self
    // }

    // pub fn service_by_type<T: AppService_2>(&self) -> Result<&T> {
    //     Ok(self.service_pool.get_by_type::<T>()?)
    // }
}
