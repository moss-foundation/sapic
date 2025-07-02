use moss_app::app::App;
use moss_collection::models::{
    events::StreamEntriesEvent,
    operations::{
        CreateEntryInput, CreateEntryOutput, DeleteEntryInput, DeleteEntryOutput,
        StreamEntriesOutput,
    },
};
use moss_common::api::OperationOptionExt;
use moss_tauri::{TauriError, TauriResult};
use tauri::{Runtime as TauriRuntime, State, Window, ipc::Channel as TauriChannel};

use crate::constants::DEFAULT_COMMAND_TIMEOUT;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn create_collection_entry<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    collection_id: String,
    input: CreateEntryInput,
) -> TauriResult<CreateEntryOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let (mut workspace, ctx) = app
            .workspace_mut()
            .await
            .map_err_as_failed_precondition("No active workspace")?;

        let collections = workspace.collections_mut(&ctx).await?;
        let collection_item = collections
            .get(&collection_id)
            .map_err_as_not_found("Collection not found")?;
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
    collection_id: String,
    input: DeleteEntryInput,
) -> TauriResult<DeleteEntryOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let (mut workspace, ctx) = app
            .workspace_mut()
            .await
            .map_err_as_failed_precondition("No active workspace")?;

        let collections = workspace.collections_mut(&ctx).await?;
        let collection_item = collections
            .get(&collection_id)
            .map_err_as_not_found("Collection not found")?;
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
    collection_id: String,
    channel: TauriChannel<StreamEntriesEvent>,
) -> TauriResult<StreamEntriesOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let (workspace, ctx) = app
            .workspace()
            .await
            .map_err_as_failed_precondition("No active workspace")?;

        let collections = workspace.collections(&ctx).await?;
        let collection_item = collections
            .get(&collection_id)
            .map_err_as_not_found("Collection not found")?;

        let collection_item_lock = collection_item.read().await;
        collection_item_lock
            .stream_entries(channel)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}
