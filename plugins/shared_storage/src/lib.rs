mod models;
mod service;

pub mod provider;

use joinerror::OptionExt;
use moss_api::TauriResult;
use moss_storage2::Storage;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tauri::{
    AppHandle, Emitter, Runtime, Window, WindowEvent,
    plugin::{Builder, TauriPlugin},
};
use tracing::instrument;

use crate::{
    models::{operations::GetItemOutput, primitives::Scope},
    provider::{GenericAppHandle, PROVIDER_CALLBACK},
};

pub fn init<
    R: Runtime,
    F: Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn Storage>> + Send + Sync + 'static,
>(
    f: F,
) -> TauriPlugin<R> {
    let _ = PROVIDER_CALLBACK.set(Arc::new(f));
    Builder::new("shared_storage_service")
        .invoke_handler(tauri::generate_handler![get_item])
        .build()
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn get_item<'a, R: tauri::Runtime>(app_handle: AppHandle<R>) -> TauriResult<GetItemOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage = provider(&GenericAppHandle::new(app_handle));

    Ok(GetItemOutput {
        key: "test".to_string(),
        value: JsonValue::String("test".to_string()),
        scope: Scope::App,
    })
}
