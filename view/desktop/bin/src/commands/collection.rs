use moss_api::{TauriError, TauriResult};
use moss_app::app::App;
use moss_collection::models::{events::*, operations::*};
use moss_workspace::models::primitives::CollectionId;
use tauri::{State, Window, ipc::Channel as TauriChannel};

use crate::{TauriAsyncRuntime, commands::Options};

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_collection_entry<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAsyncRuntime<R>>>,
    window: Window<R>,
    collection_id: CollectionId,
    input: CreateEntryInput,
    options: Options,
) -> TauriResult<CreateEntryOutput> {
    super::with_collection_timeout(
        ctx.inner(),
        app,
        collection_id,
        options,
        |ctx, collection| async move {
            collection
                .create_entry(&ctx, input)
                .await
                .map_err(TauriError::OperationError)
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_collection_entry<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAsyncRuntime<R>>>,
    window: Window<R>,
    collection_id: CollectionId,
    input: DeleteEntryInput,
    options: Options,
) -> TauriResult<DeleteEntryOutput> {
    super::with_collection_timeout(
        ctx.inner(),
        app,
        collection_id,
        options,
        |ctx, collection| async move {
            collection
                .delete_entry(&ctx, input)
                .await
                .map_err(TauriError::OperationError)
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_collection_entry<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAsyncRuntime<R>>>,
    window: Window<R>,
    collection_id: CollectionId,
    input: UpdateEntryInput,
    options: Options,
) -> TauriResult<UpdateEntryOutput> {
    super::with_collection_timeout(
        ctx.inner(),
        app,
        collection_id,
        options,
        |ctx, collection| async move {
            collection
                .update_entry(&ctx, input)
                .await
                .map_err(TauriError::OperationError)
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn batch_update_collection_entry<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAsyncRuntime<R>>>,
    window: Window<R>,
    channel: TauriChannel<BatchUpdateEntryEvent>,
    collection_id: CollectionId,
    input: BatchUpdateEntryInput,
    options: Options,
) -> TauriResult<BatchUpdateEntryOutput> {
    super::with_collection_timeout(
        ctx.inner(),
        app,
        collection_id,
        options,
        |ctx, collection| async move {
            collection
                .batch_update_entry(&ctx, input, channel)
                .await
                .map_err(TauriError::OperationError)
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_collection_entries<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAsyncRuntime<R>>>,
    window: Window<R>,
    collection_id: CollectionId,
    input: Option<StreamEntriesInput>, // FIXME: this needs to be optional because the frontend doesn't send it yet
    channel: TauriChannel<StreamEntriesEvent>,
    options: Options,
) -> TauriResult<StreamEntriesOutput> {
    super::with_collection_timeout(
        ctx.inner(),
        app,
        collection_id,
        options,
        |ctx, collection| async move {
            // FIXME: temporary hack
            let input = if let Some(input) = input {
                input
            } else {
                StreamEntriesInput::LoadRoot
            };

            collection
                .stream_entries(&ctx, channel, input)
                .await
                .map_err(TauriError::OperationError)
        },
    )
    .await
}
