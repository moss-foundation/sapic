use moss_api::TauriResult;
use moss_workspace::{
    api::BatchUpdateCollectionOp,
    models::{events::*, operations::*},
};
use tauri::{State, Window, ipc::Channel as TauriChannel};

use crate::commands::primitives::*;

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_workspace_state<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: UpdateStateInput,
    options: Options,
) -> TauriResult<()> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.update_state(&ctx, input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn describe_workspace_state<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> TauriResult<DescribeStateOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.describe_state(&ctx).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_environments<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    channel: TauriChannel<StreamEnvironmentsEvent>,
    options: Options,
) -> TauriResult<StreamEnvironmentsOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.stream_environments(&ctx, channel).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_collections<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    channel: TauriChannel<StreamCollectionsEvent>,
    options: Options,
) -> TauriResult<StreamCollectionsOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.stream_collections(&ctx, channel).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_collection<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: CreateCollectionInput,
    options: Options,
) -> TauriResult<CreateCollectionOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.create_collection(&ctx, &input).await
    })
    .await
}

#[allow(dead_code)]
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn import_collection<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: ImportCollectionInput,
    options: Options,
) -> TauriResult<ImportCollectionOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.import_collection(&ctx, &input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_collection<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: DeleteCollectionInput,
    options: Options,
) -> TauriResult<DeleteCollectionOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.delete_collection(&ctx, &input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_collection<'a, R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: App<'a, R>,
    window: Window<R>,
    input: UpdateCollectionInput,
    options: Options,
) -> TauriResult<UpdateCollectionOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.update_collection(&ctx, input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn batch_update_collection<'a, R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: App<'a, R>,
    window: Window<R>,
    input: BatchUpdateCollectionInput,
    options: Options,
) -> TauriResult<BatchUpdateCollectionOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.batch_update_collection(&ctx, input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_environment<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: CreateEnvironmentInput,
    options: Options,
) -> TauriResult<CreateEnvironmentOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.create_environment(&ctx, input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_environment<'a, R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: App<'a, R>,
    window: Window<R>,
    input: UpdateEnvironmentInput,
    options: Options,
) -> TauriResult<UpdateEnvironmentOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.update_environment(&ctx, input).await
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_environment<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: DeleteEnvironmentInput,
    options: Options,
) -> TauriResult<DeleteEnvironmentOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace.delete_environment(&ctx, input).await
    })
    .await
}
