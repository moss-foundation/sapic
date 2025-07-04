use moss_api::{self as api, TauriError, TauriResult};
use moss_app::app::App;
use moss_common::api::OperationOptionExt;
use moss_workspace::models::{events::*, operations::*};
use tauri::{Runtime as TauriRuntime, State, Window, ipc::Channel as TauriChannel};

use crate::commands::Options;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_workspace_state<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: UpdateStateInput,
    options: Options,
) -> TauriResult<()> {
    api::with_timeout(options, async move {
        let (workspace, _ctx) = app
            .workspace()
            .await
            .map_err_as_failed_precondition("No active workspace")?;

        workspace
            .update_state(input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn describe_workspace_state<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    options: Options,
) -> TauriResult<DescribeStateOutput> {
    api::with_timeout(options, async move {
        let (workspace, _ctx) = app
            .workspace()
            .await
            .map_err_as_failed_precondition("No active workspace")?;
        workspace
            .describe_state()
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_workspace_environments<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    channel: TauriChannel<StreamEnvironmentsEvent>,
    options: Options,
) -> TauriResult<()> {
    api::with_timeout(options, async move {
        let (workspace, ctx) = app
            .workspace()
            .await
            .map_err_as_failed_precondition("No active workspace")?;
        workspace
            .stream_environments(&ctx, channel)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_collections<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    channel: TauriChannel<StreamCollectionsEvent>,
    options: Options,
) -> TauriResult<StreamCollectionsOutput> {
    api::with_timeout(options, async move {
        let (workspace, ctx) = app
            .workspace()
            .await
            .map_err_as_failed_precondition("No active workspace")?;
        workspace
            .stream_collections(&ctx, channel)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn create_collection<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: CreateCollectionInput,
    options: Options,
) -> TauriResult<CreateCollectionOutput> {
    api::with_timeout(options, async move {
        let (mut workspace, ctx) = app
            .workspace_mut()
            .await
            .map_err_as_failed_precondition("No active workspace")?;
        workspace
            .create_collection(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn delete_collection<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: DeleteCollectionInput,
    options: Options,
) -> TauriResult<DeleteCollectionOutput> {
    api::with_timeout(options, async move {
        let (mut workspace, ctx) = app
            .workspace_mut()
            .await
            .map_err_as_failed_precondition("No active workspace")?;
        workspace
            .delete_collection(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}
