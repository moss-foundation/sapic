mod models;
pub mod provider;

use joinerror::{OptionExt, ResultExt};
use moss_api::TauriResult;
use moss_storage2::Storage;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tauri::{
    AppHandle, Runtime,
    plugin::{Builder, TauriPlugin},
};
use tracing::instrument;

use crate::{
    models::operations::{
        GetItemInput, GetItemOutput, PutItemInput, PutItemOutput, RemoveItemInput, RemoveItemOutput,
    },
    provider::{GenericAppHandle, PROVIDER_CALLBACK},
};

pub fn init<
    R: Runtime,
    F: Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn Storage>> + Send + Sync + 'static,
>(
    f: F,
) -> TauriPlugin<R> {
    let _ = PROVIDER_CALLBACK.set(Arc::new(f));
    Builder::new("shared-storage")
        .invoke_handler(tauri::generate_handler![get_item, put_item, remove_item])
        .build()
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn get_item<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: GetItemInput,
) -> TauriResult<GetItemOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let value = storage
        .get(input.scope.clone().into(), &input.key)
        .await?
        .ok_or_join_err::<()>("item not found")?;

    Ok(GetItemOutput {
        key: input.key.clone(),
        value,
        scope: input.scope,
    })
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn put_item<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: PutItemInput,
) -> TauriResult<PutItemOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let _ = storage
        .put(input.scope.into(), &input.key, input.value)
        .await
        .join_err::<()>("failed to put item")?;

    Ok(PutItemOutput {})
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn remove_item<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: RemoveItemInput,
) -> TauriResult<RemoveItemOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let _ = storage
        .remove(input.scope.into(), &input.key)
        .await
        .join_err::<()>("failed to remove item")?;

    Ok(RemoveItemOutput {})
}
