use anyhow::anyhow;
use moss_app::{app::App, context::AppContext};
use moss_collection::models::{
    events::StreamEntriesEvent,
    operations::{
        CreateEntryInput, CreateEntryOutput, DeleteEntryInput, DeleteEntryOutput,
        StreamEntriesOutput,
    },
};
use moss_tauri::{TauriError, TauriResult};
use tauri::{Runtime as TauriRuntime, State, Window, ipc::Channel as TauriChannel};
use uuid::Uuid;

use crate::constants::DEFAULT_COMMAND_TIMEOUT;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn create_collection_entry<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    collection_id: Uuid,
    input: CreateEntryInput,
) -> TauriResult<CreateEntryOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        let current_workspace = workbench
            .active_workspace()
            .ok_or_else(|| TauriError::Other(anyhow!("No active workspace")))?; // TODO: improve error handling

        let collections = current_workspace.collections(&ctx).await?;
        let collections_lock = collections.write().await;
        let collection_item = collections_lock.get(&collection_id).ok_or_else(|| {
            TauriError::Other(anyhow!("Collection not found")) // TODO: improve error handling
        })?;
        let mut collection_item_lock = collection_item.write().await;
        collection_item_lock
            .create_entry(input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn delete_collection_entry<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    collection_id: Uuid,
    input: DeleteEntryInput,
) -> TauriResult<DeleteEntryOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        let current_workspace = workbench
            .active_workspace()
            .ok_or_else(|| TauriError::Other(anyhow!("No active workspace")))?; // TODO: improve error handling

        let collections = current_workspace.collections(&ctx).await?;
        let collections_lock = collections.write().await;
        let collection_item = collections_lock.get(&collection_id).ok_or_else(|| {
            TauriError::Other(anyhow!("Collection not found")) // TODO: improve error handling
        })?;
        let mut collection_item_lock = collection_item.write().await;
        collection_item_lock
            .delete_entry(input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_collection_entries<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    collection_id: Uuid,
    channel: TauriChannel<StreamEntriesEvent>,
) -> TauriResult<StreamEntriesOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        let current_workspace = workbench
            .active_workspace()
            .ok_or_else(|| TauriError::Other(anyhow!("No active workspace")))?; // TODO: improve error handling

        let collections = current_workspace.collections(&ctx).await?;
        let collections_lock = collections.read().await;
        let collection_item = collections_lock.get(&collection_id).ok_or_else(|| {
            TauriError::Other(anyhow!("Collection not found")) // TODO: improve error handling
        })?;
        let collection_item_lock = collection_item.read().await;
        collection_item_lock
            .stream_entries(channel)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}
