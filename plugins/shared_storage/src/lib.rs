mod models;

use joinerror::{OptionExt, ResultExt};
use moss_api::TauriResult;
use moss_applib::GenericAppHandle;
use moss_storage2::Storage;
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};
use tauri::{
    AppHandle, Runtime,
    plugin::{Builder, TauriPlugin},
};
use tracing::instrument;

use crate::models::operations::*;

pub(crate) type ProviderCallback =
    Arc<dyn Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn Storage>> + Send + Sync>;

pub(crate) static PROVIDER_CALLBACK: OnceLock<ProviderCallback> = OnceLock::new();

pub fn init<
    R: Runtime,
    F: Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn Storage>> + Send + Sync + 'static,
>(
    f: F,
) -> TauriPlugin<R> {
    let _ = PROVIDER_CALLBACK.set(Arc::new(f));
    Builder::new("shared-storage")
        .invoke_handler(tauri::generate_handler![
            get_item,
            put_item,
            remove_item,
            batch_get_item,
            batch_put_item,
            batch_remove_item
        ])
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
    storage
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
    let value = storage
        .remove(input.scope.clone().into(), &input.key)
        .await
        .join_err::<()>("failed to remove item")?;

    Ok(RemoveItemOutput {
        scope: input.scope,
        value,
    })
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn batch_put_item<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: BatchPutItemInput,
) -> TauriResult<BatchPutItemOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let items: Vec<(&str, JsonValue)> = input
        .items
        .iter()
        .map(|(k, v)| (k.as_str(), v.clone()))
        .collect();
    storage
        .put_batch(input.scope.into(), &items)
        .await
        .join_err::<()>("failed to batch put items")?;

    Ok(BatchPutItemOutput {})
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn batch_remove_item<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: BatchRemoveItemInput,
) -> TauriResult<BatchRemoveItemOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let items = storage
        .remove_batch(
            input.scope.clone().into(),
            &input.keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>(),
        )
        .await
        .join_err::<()>("failed to batch remove items")?;

    let items_map: HashMap<String, Option<JsonValue>> = items.into_iter().collect();

    Ok(BatchRemoveItemOutput {
        scope: input.scope,
        items: items_map,
    })
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn batch_get_item<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: BatchGetItemInput,
) -> TauriResult<BatchGetItemOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let items = storage
        .get_batch(
            input.scope.clone().into(),
            &input.keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>(),
        )
        .await
        .join_err::<()>("failed to batch get items")?;

    let items_map: HashMap<String, Option<JsonValue>> = items.into_iter().collect();

    Ok(BatchGetItemOutput {
        scope: input.scope,
        items: items_map,
    })
}
