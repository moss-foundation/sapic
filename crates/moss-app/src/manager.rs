use super::service::{AppService, InstantiationType, ServiceCollection, ServiceHandle};
use anyhow::Result;
use moss_db::encrypted_bincode_store::EncryptedBincodeStore;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MockVault {}

pub struct AppManager {
    services: ServiceCollection,
    v_store: EncryptedBincodeStore<'static, &'static str, MockVault>,
    // TODO: Registry
}

unsafe impl Send for AppManager {}
unsafe impl Sync for AppManager {}

impl AppManager {
    pub fn new(
        app_handle: AppHandle,
        v_store: EncryptedBincodeStore<'static, &'static str, MockVault>,
    ) -> Self {
        Self {
            services: ServiceCollection::new(app_handle),
            v_store,
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
