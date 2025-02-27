use anyhow::Result;
use tauri::AppHandle;

use super::service::{AppService, InstantiationType, ServiceCollection, ServiceHandle};

pub struct AppManager {
    services: ServiceCollection,
    // TODO: Registry
}

unsafe impl Send for AppManager {}
unsafe impl Sync for AppManager {}

impl AppManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            services: ServiceCollection::new(app_handle),
        }
    }

    pub fn with_service<T, F>(self, service: F, activation_type: InstantiationType) -> Self
    where
        T: AppService + 'static,
        F: FnOnce(&AppHandle) -> T + 'static,
    {
        self.services.register(service, activation_type);
        self
    }

    pub fn service<T: AppService>(&self) -> Result<ServiceHandle<T>> {
        self.services.get()
    }
}
