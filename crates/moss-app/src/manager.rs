use tauri::{AppHandle, Runtime as TauriRuntime};

use crate::service::prelude::ServicePool;

pub struct AppManager<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    service_pool: ServicePool<R>,
}

impl<R: TauriRuntime> AppManager<R> {
    pub fn new(app_handle: AppHandle<R>, service_pool: ServicePool<R>) -> Self {
        Self {
            app_handle,
            service_pool,
        }
    }

    pub fn services(&self) -> &ServicePool<R> {
        &self.service_pool
    }

    pub fn app_handle(&self) -> &AppHandle<R> {
        &self.app_handle
    }
}
