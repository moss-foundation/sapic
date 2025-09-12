use moss_api::TauriResult;
use moss_collection::models::{events::*, operations::*};
use moss_workspace::models::primitives::CollectionId;
use tauri::{Window, ipc::Channel as TauriChannel};

use crate::commands::primitives::*;

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_collection_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
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
        |ctx, _, collection| async move { collection.create_entry(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_collection_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
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
        |ctx, _, collection| async move { collection.delete_entry(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_collection_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
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
        |ctx, _, collection| async move { collection.update_entry(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn batch_create_collection_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    collection_id: CollectionId,
    input: BatchCreateEntryInput,
    options: Options,
) -> TauriResult<BatchCreateEntryOutput> {
    super::with_collection_timeout(
        ctx.inner(),
        app,
        collection_id,
        options,
        |ctx, _, collection| async move { collection.batch_create_entry(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn batch_update_collection_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
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
        |ctx, _, collection| async move { collection.batch_update_entry(&ctx, input, channel).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_collection_entries<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    collection_id: CollectionId,
    input: StreamEntriesInput,
    channel: TauriChannel<StreamEntriesEvent>,
    options: Options,
) -> TauriResult<StreamEntriesOutput> {
    super::with_collection_timeout(
        ctx.inner(),
        app,
        collection_id,
        options,
        |ctx, app_delegate, collection| async move {
            collection
                .stream_entries(&ctx, &app_delegate, channel, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn execute_vcs_operation<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    collection_id: CollectionId,
    input: ExecuteVcsOperationInput,
    options: Options,
) -> TauriResult<ExecuteVcsOperationOutput> {
    super::with_collection_timeout(
        ctx.inner(),
        app,
        collection_id,
        options,
        |ctx, _app_delegate, collection| async move {
            collection.execute_vcs_operation(&ctx, input).await
        },
    )
    .await
}
