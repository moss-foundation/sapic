mod models;

use joinerror::{OptionExt, ResultExt};
use moss_applib::{GenericAppHandle, task::Task};
use moss_logging::session;
use moss_storage2::{FlushMode, Storage, StorageCapabilities};
use sapic_ipc::TauriResult;
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
    time::Duration,
};
use tauri::{
    AppHandle, Manager, RunEvent, Runtime, WindowEvent,
    plugin::{Builder, TauriPlugin},
};
use tracing::instrument;

use crate::models::operations::*;

pub(crate) type ProviderCallback =
    Arc<dyn Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn Storage>> + Send + Sync>;

pub(crate) static PROVIDER_CALLBACK: OnceLock<ProviderCallback> = OnceLock::new();

/// Minimum interval between checkpoint operations
const CHECKPOINT_INTERVAL: Duration = Duration::from_secs(5 * 60); // 5 minutes

pub fn init<
    R: Runtime,
    F: Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn Storage>> + Send + Sync + 'static,
>(
    f: F,
) -> TauriPlugin<R> {
    let _ = PROVIDER_CALLBACK.set(Arc::new(f));

    Builder::new("shared-storage")
        .on_event(move |app_handle, event| match event {
            RunEvent::WindowEvent { label, event, .. } => match event {
                WindowEvent::Focused(focused) if !focused => {
                    let app_handle_clone = app_handle.clone();

                    let _ = Task::with_timeout(Duration::from_secs(60), async move {
                        on_event_window_focused(app_handle_clone).await
                    })
                    .detach()
                    .log_if_err("background database checkpoint");
                }
                WindowEvent::CloseRequested { api, .. } => {
                    // Prevent window from closing until flush completes
                    api.prevent_close();

                    let app_handle_clone = app_handle.clone();
                    let label_clone = label.clone();

                    tokio::spawn(async move {
                        let flush_timeout = Duration::from_secs(10);
                        let flush_result = tokio::time::timeout(
                            flush_timeout,
                            on_event_window_close_requested(app_handle_clone.clone()),
                        )
                        .await;

                        match flush_result {
                            Ok(Ok(())) => {
                                session::debug!(
                                    "Database flush completed successfully before window close"
                                );
                            }
                            Ok(Err(e)) => {
                                session::error!(format!(
                                    "Failed to flush database on close: {}",
                                    e
                                ));
                            }
                            Err(_) => {
                                session::error!(format!(
                                    "Database flush timed out after {:?}, closing window anyway",
                                    flush_timeout
                                ));
                            }
                        }

                        if let Some(window) = app_handle_clone.get_webview_window(&label_clone) {
                            if let Err(e) = window.destroy() {
                                session::error!(format!("Failed to close window: {}", e));
                            }
                        }
                    });
                }
                _ => (),
            },
            _ => (),
        })
        .invoke_handler(tauri::generate_handler![
            get_item,
            put_item,
            remove_item,
            batch_get_item,
            batch_put_item,
            batch_remove_item,
            batch_get_item_by_prefix,
            batch_remove_item_by_prefix
        ])
        .build()
}

async fn on_event_window_close_requested<R: Runtime>(
    app_handle: AppHandle<R>,
) -> joinerror::Result<()> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let storage_capabilities: Arc<dyn StorageCapabilities> = storage.capabilities().await;

    storage_capabilities
        .flush(FlushMode::Force)
        .await
        .join_err::<()>("failed to complete database force flush")?;

    Ok(())
}

/// Attempts to perform a database checkpoint if enough time has passed since the last one
async fn on_event_window_focused<R: Runtime>(app_handle: AppHandle<R>) -> joinerror::Result<()> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let storage_capabilities: Arc<dyn StorageCapabilities> = storage.capabilities().await;

    if let Some(last_time) = storage_capabilities.last_checkpoint().await {
        let elapsed = last_time.elapsed();

        if elapsed < CHECKPOINT_INTERVAL {
            return Ok(());
        }
    }

    storage_capabilities
        .flush(FlushMode::Checkpoint)
        .await
        .join_err::<()>("failed to complete database checkpoint")?;

    Ok(session::debug!("Completed background database checkpoint"))
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

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn batch_get_item_by_prefix<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: BatchGetItemByPrefixInput,
) -> TauriResult<BatchGetItemByPrefixOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let items = storage
        .get_batch_by_prefix(input.scope.clone().into(), &input.prefix)
        .await
        .join_err::<()>("failed to batch get item by prefix")?;

    let items_map: HashMap<String, JsonValue> = items.into_iter().collect();
    Ok(BatchGetItemByPrefixOutput {
        scope: input.scope,
        items: items_map,
    })
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn batch_remove_item_by_prefix<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: BatchRemoveItemByPrefixInput,
) -> TauriResult<BatchRemoveItemByPrefixOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("storage provider not found")?;

    let storage: Arc<dyn Storage> = provider(&GenericAppHandle::new(app_handle))?;
    let items = storage
        .remove_batch_by_prefix(input.scope.clone().into(), &input.prefix)
        .await
        .join_err::<()>("failed to batch remove item by prefix")?;

    let items_map: HashMap<String, JsonValue> = items.into_iter().collect();
    Ok(BatchRemoveItemByPrefixOutput {
        scope: input.scope,
        items: items_map,
    })
}
