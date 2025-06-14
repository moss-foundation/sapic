use moss_app::context::Context;
use moss_tauri::{TauriError, TauriResult};
use moss_workbench::models::operations::*;
use std::time::Duration;
use tauri::{Runtime as TauriRuntime, Window};

use crate::{commands::ReadWorkbench, primitives::AppState};

#[tauri::command(async)]
#[instrument(level = "trace", skip(state), fields(window = window.label()))]
pub async fn create_workspace<R: TauriRuntime>(
    state: AppState<'_, R>,
    window: Window<R>,
    input: CreateWorkspaceInput,
) -> TauriResult<CreateWorkspaceOutput> {
    let task = state.spawn::<CreateWorkspaceOutput, TauriError, _, _>(
        move |ctx| async move {
            let workbench = ctx.workbench();
            let workspace_output = workbench
                .create_workspace(&ctx, &input)
                .await
                .map_err(|err| TauriError(format!("Failed to create workspace: {}", err)))?;

            Ok(workspace_output)
        },
        Some(Duration::from_secs(30)),
    );

    Ok(task.await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state), fields(window = window.label()))]
pub async fn list_workspaces<R: TauriRuntime>(
    state: AppState<'_, R>,
    window: Window<R>,
) -> TauriResult<ListWorkspacesOutput> {
    let task = state.spawn::<ListWorkspacesOutput, TauriError, _, _>(
        move |ctx| async move {
            let workbench = ctx.workbench();
            let workspaces = workbench.list_workspaces(&ctx).await?;
            Ok(workspaces)
        },
        Some(Duration::from_secs(30)),
    );

    Ok(task.await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state), fields(window = window.label()))]
pub async fn delete_workspace<R: TauriRuntime>(
    state: AppState<'_, R>,
    window: Window<R>,
    input: DeleteWorkspaceInput,
) -> TauriResult<()> {
    let task = state.spawn::<(), TauriError, _, _>(
        move |ctx| async move {
            let workbench = ctx.workbench();
            workbench.delete_workspace(&ctx, &input).await?;
            Ok(())
        },
        Some(Duration::from_secs(30)),
    );

    Ok(task.await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state), fields(window = window.label()))]
pub async fn open_workspace<R: TauriRuntime>(
    state: AppState<'_, R>,
    window: Window<R>,
    input: OpenWorkspaceInput,
) -> TauriResult<OpenWorkspaceOutput> {
    let task = state.spawn::<OpenWorkspaceOutput, TauriError, _, _>(
        move |ctx| async move {
            let workbench = ctx.workbench();
            let workspace_output = workbench.open_workspace(&ctx, &input).await?;
            Ok(workspace_output)
        },
        Some(Duration::from_secs(30)),
    );

    Ok(task.await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state), fields(window = window.label()))]
pub async fn update_workspace<R: TauriRuntime>(
    state: AppState<'_, R>,
    window: Window<R>,
    input: UpdateWorkspaceInput,
) -> TauriResult<()> {
    let task = state.spawn::<(), TauriError, _, _>(
        move |ctx| async move {
            let workbench = ctx.workbench();
            workbench.update_workspace(&ctx, &input).await?;
            Ok(())
        },
        Some(Duration::from_secs(30)),
    );

    Ok(task.await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state), fields(window = window.label()))]
pub async fn describe_workbench_state<R: TauriRuntime>(
    state: AppState<'_, R>,
    window: Window<R>,
) -> TauriResult<DescribeWorkbenchStateOutput> {
    let task = state.spawn::<DescribeWorkbenchStateOutput, TauriError, _, _>(
        move |ctx| async move {
            let workbench = ctx.workbench();
            let state = workbench.describe_state().await?;
            Ok(state)
        },
        Some(Duration::from_secs(30)),
    );

    Ok(task.await?)
}
