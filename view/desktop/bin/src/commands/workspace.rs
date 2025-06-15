use anyhow::anyhow;
use moss_app::{app::App, context::AppContext};
use moss_tauri::{TauriError, TauriResult};
use moss_workspace::models::{
    events::{StreamCollectionsEvent, StreamEnvironmentsEvent},
    operations::{
        CreateCollectionInput, CreateCollectionOutput, DeleteCollectionInput,
        DeleteCollectionOutput, DescribeStateOutput, UpdateStateInput,
    },
};
use tauri::{Runtime as TauriRuntime, State, Window, ipc::Channel as TauriChannel};

use crate::constants::DEFAULT_COMMAND_TIMEOUT;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_workspace_state<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: UpdateStateInput,
) -> TauriResult<()> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let _ctx = AppContext::from(&app);
        let workbench = app.workbench();
        let current_workspace = workbench
            .active_workspace()
            .ok_or_else(|| TauriError::Other(anyhow!("No active workspace")))?; // TODO: improve error handling
        current_workspace
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
) -> TauriResult<DescribeStateOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let _ctx = AppContext::from(&app);
        let workbench = app.workbench();
        let current_workspace = workbench
            .active_workspace()
            .ok_or_else(|| TauriError::Other(anyhow!("No active workspace")))?; // TODO: improve error handling
        current_workspace
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
) -> TauriResult<()> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        let current_workspace = workbench
            .active_workspace()
            .ok_or_else(|| TauriError::Other(anyhow!("No active workspace")))?; // TODO: improve error handling
        current_workspace
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
) -> TauriResult<()> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        let current_workspace = workbench
            .active_workspace()
            .ok_or_else(|| TauriError::Other(anyhow!("No active workspace")))?; // TODO: improve error handling
        current_workspace
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
) -> TauriResult<CreateCollectionOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        let current_workspace = workbench
            .active_workspace()
            .ok_or_else(|| TauriError::Other(anyhow!("No active workspace")))?; // TODO: improve error handling
        current_workspace
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
) -> TauriResult<DeleteCollectionOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        let current_workspace = workbench
            .active_workspace()
            .ok_or_else(|| TauriError::Other(anyhow!("No active workspace")))?; // TODO: improve error handling
        current_workspace
            .delete_collection(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}
