use moss_api::{self as api, TauriError, TauriResult};
use moss_app::app::App;
use moss_collection::models::{events::*, operations::*};
use moss_common::api::OperationOptionExt;
use tauri::{Runtime as TauriRuntime, State, Window, ipc::Channel as TauriChannel};
use uuid::Uuid;

use crate::commands::Options;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn create_collection_entry<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    collection_id: Uuid,
    input: CreateEntryInput,
    options: Options,
) -> TauriResult<CreateEntryOutput> {
    api::with_timeout(options, async move {
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
    collection_id: Uuid,
    input: DeleteEntryInput,
    options: Options,
) -> TauriResult<DeleteEntryOutput> {
    api::with_timeout(options, async move {
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
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_collection_entry<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    collection_id: Uuid,
    input: UpdateEntryInput,
    options: Options,
) -> TauriResult<UpdateEntryOutput> {
    api::with_timeout(options, async move {
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
            .update_entry(input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label(), channel = channel.id()))]
pub async fn batch_update_collection_entry<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    channel: TauriChannel<BatchUpdateEntryEvent>,
    collection_id: Uuid,
    input: BatchUpdateEntryInput,
    options: Options,
) -> TauriResult<BatchUpdateEntryOutput> {
    api::with_timeout(options, async move {
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
            .batch_update_entry(input, channel)
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
    input: Option<StreamEntriesInput>, // FIXME: this needs to be optional because the frontend doesn't send it yet
    channel: TauriChannel<StreamEntriesEvent>,
    options: Options,
) -> TauriResult<StreamEntriesOutput> {
    api::with_timeout(options, async move {
        let (workspace, ctx) = app
            .workspace()
            .await
            .map_err_as_failed_precondition("No active workspace")?;

        let collections = workspace.collections(&ctx).await?;
        let collection_item = collections
            .get(&collection_id)
            .map_err_as_not_found("Collection not found")?;

        // FIXME: temporary hack
        let input = if let Some(input) = input {
            input
        } else {
            StreamEntriesInput { paths: Vec::new() }
        };

        let collection_item_lock = collection_item.read().await;
        collection_item_lock
            .stream_entries(channel, input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}
