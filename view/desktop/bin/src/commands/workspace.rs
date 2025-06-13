use moss_app::context::Context;
use moss_tauri::{TauriError, TauriResult};
use moss_workspace::models::{
    events::StreamEnvironmentsEvent,
    operations::{DescribeStateOutput, UpdateStateInput},
};
use tauri::{Runtime as TauriRuntime, Window, ipc::Channel as TauriChannel};

use crate::{commands::ReadWorkbench, primitives::AppState};

#[tauri::command(async)]
#[instrument(level = "trace", skip(state), fields(window = window.label()))]
pub async fn update_workspace_state<R: TauriRuntime>(
    state: AppState<'_, R>,
    window: Window<R>,
    input: UpdateStateInput,
) -> TauriResult<()> {
    let task = state.spawn::<(), TauriError, _, _>(
        move |ctx| async move {
            let workbench = ctx.workbench();
            let current_workspace = workbench
                .active_workspace()
                .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling
            current_workspace.update_state(input).await?;
            Ok(())
        },
        None, // TODO: add timeout
    );

    match task.await {
        moss_app::context::TaskResult::Ok(result) => Ok(result),
        moss_app::context::TaskResult::Err(err) => Err(err),
        moss_app::context::TaskResult::Timeout => Err(TauriError("Task timed out".to_string())),
        moss_app::context::TaskResult::Cancelled => {
            Err(TauriError("Task was cancelled".to_string()))
        }
    }
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state), fields(window = window.label()))]
pub async fn describe_workspace_state<R: TauriRuntime>(
    state: AppState<'_, R>,
    window: Window<R>,
) -> TauriResult<DescribeStateOutput> {
    let task = state.spawn::<DescribeStateOutput, TauriError, _, _>(
        move |ctx| async move {
            let workbench = ctx.workbench();
            let current_workspace = workbench
                .active_workspace()
                .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling
            let output = current_workspace.describe_state().await?;
            Ok(output)
        },
        None, // TODO: add timeout
    );

    match task.await {
        moss_app::context::TaskResult::Ok(result) => {
            dbg!(&result);
            Ok(result)
        }
        moss_app::context::TaskResult::Err(err) => Err(err),
        moss_app::context::TaskResult::Timeout => Err(TauriError("Task timed out".to_string())),
        moss_app::context::TaskResult::Cancelled => {
            Err(TauriError("Task was cancelled".to_string()))
        }
    }
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_workspace_environments<R: TauriRuntime>(
    state: AppState<'_, R>,
    window: Window<R>,
    channel: TauriChannel<StreamEnvironmentsEvent>,
) -> TauriResult<()> {
    let task = state.spawn::<(), TauriError, _, _>(
        move |ctx| async move {
            let workbench = ctx.workbench();
            let current_workspace = workbench
                .active_workspace()
                .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling
            current_workspace
                .stream_environments(&*ctx, channel)
                .await?;
            Ok(())
        },
        None, // TODO: add timeout
    );

    match task.await {
        moss_app::context::TaskResult::Ok(result) => Ok(result),
        moss_app::context::TaskResult::Err(err) => Err(err),
        moss_app::context::TaskResult::Timeout => Err(TauriError("Task timed out".to_string())),
        moss_app::context::TaskResult::Cancelled => {
            Err(TauriError("Task was cancelled".to_string()))
        }
    }
}
