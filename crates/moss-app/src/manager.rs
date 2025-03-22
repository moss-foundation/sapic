use tauri::AppHandle;

use crate::service::prelude::ServicePool;

pub struct AppManager {
    app_handle: AppHandle,
    service_pool: ServicePool,
}

impl AppManager {
    pub fn new(app_handle: AppHandle, service_pool: ServicePool) -> Self {
        Self {
            app_handle,
            service_pool,
        }
    }

    pub fn services(&self) -> &ServicePool {
        &self.service_pool
    }

    pub fn app_handle(&self) -> &AppHandle {
        &self.app_handle
    }
}
