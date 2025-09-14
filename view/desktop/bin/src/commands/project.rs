use moss_api::TauriResult;
use moss_project::models::{events::*, operations::*};
use moss_workspace::models::primitives::ProjectId;
use tauri::{Window, ipc::Channel as TauriChannel};

use crate::commands::primitives::*;

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_project_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: CreateEntryInput,
    options: Options,
) -> TauriResult<CreateEntryOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        project_id,
        options,
        |ctx, _, project| async move { project.create_entry(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_project_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: DeleteEntryInput,
    options: Options,
) -> TauriResult<DeleteEntryOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        project_id,
        options,
        |ctx, _, project| async move { project.delete_entry(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_project_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: UpdateEntryInput,
    options: Options,
) -> TauriResult<UpdateEntryOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        project_id,
        options,
        |ctx, _, project| async move { project.update_entry(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn batch_create_project_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: BatchCreateEntryInput,
    options: Options,
) -> TauriResult<BatchCreateEntryOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        project_id,
        options,
        |ctx, _, project| async move { project.batch_create_entry(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn batch_update_project_entry<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    channel: TauriChannel<BatchUpdateEntryEvent>,
    project_id: ProjectId,
    input: BatchUpdateEntryInput,
    options: Options,
) -> TauriResult<BatchUpdateEntryOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        project_id,
        options,
        |ctx, _, project| async move { project.batch_update_entry(&ctx, input, channel).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_project_entries<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: StreamEntriesInput,
    channel: TauriChannel<StreamEntriesEvent>,
    options: Options,
) -> TauriResult<StreamEntriesOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        project_id,
        options,
        |ctx, app_delegate, project| async move {
            project
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
        |ctx, app_delegate, collection| async move {
            collection
                .execute_vcs_operation(&ctx, &app_delegate, input)
                .await
        },
    )
    .await
}
