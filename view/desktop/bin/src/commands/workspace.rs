use moss_api::{TauriError, TauriResult};
use moss_app::app::App;
use moss_workspace::models::{events::*, operations::*};
use tauri::{State, Window, ipc::Channel as TauriChannel};

use crate::{TauriAppRuntime, commands::Options};

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_workspace_state<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAppRuntime<R>>>,
    window: Window<R>,
    input: UpdateStateInput,
    options: Options,
) -> TauriResult<()> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace
            .update_state(&ctx, input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn describe_workspace_state<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAppRuntime<R>>>,
    window: Window<R>,
    options: Options,
) -> TauriResult<DescribeStateOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace
            .describe_state(&ctx)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_workspace_environments<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAppRuntime<R>>>,
    window: Window<R>,
    channel: TauriChannel<StreamEnvironmentsEvent>,
    options: Options,
) -> TauriResult<()> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace
            .stream_environments(&ctx, channel)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_collections<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAppRuntime<R>>>,
    window: Window<R>,
    channel: TauriChannel<StreamCollectionsEvent>,
    options: Options,
) -> TauriResult<StreamCollectionsOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace
            .stream_collections(&ctx, channel)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_collection<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAppRuntime<R>>>,
    window: Window<R>,
    input: CreateCollectionInput,
    options: Options,
) -> TauriResult<CreateCollectionOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace
            .create_collection(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_collection<R: tauri::Runtime>(
    ctx: State<'_, moss_applib::context::AsyncContext>,
    app: State<'_, App<TauriAppRuntime<R>>>,
    window: Window<R>,
    input: DeleteCollectionInput,
    options: Options,
) -> TauriResult<DeleteCollectionOutput> {
    super::with_workspace_timeout(ctx.inner(), app, options, |ctx, workspace| async move {
        workspace
            .delete_collection(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
}
