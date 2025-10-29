use moss_storage2::Storage;
use std::{
    any::Any,
    sync::{Arc, OnceLock},
};
use tauri::{AppHandle, Runtime};

pub struct GenericAppHandle {
    inner: Arc<dyn Any + Send + Sync>,
}

impl GenericAppHandle {
    pub fn new<R: tauri::Runtime + 'static>(handle: AppHandle<R>) -> Self {
        Self {
            inner: Arc::new(handle),
        }
    }

    pub fn downcast<R: tauri::Runtime + 'static>(&self) -> Option<AppHandle<R>> {
        self.inner.clone().downcast_ref::<AppHandle<R>>().cloned()
    }
}

// pub trait StorageProvider {
//     fn storage(&self) -> joinerror::Result<Arc<dyn Storage>>;
// }

pub(crate) type ProviderCallback =
    Arc<dyn Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn Storage>> + Send + Sync>;

pub(crate) static PROVIDER_CALLBACK: OnceLock<ProviderCallback> = OnceLock::new();
