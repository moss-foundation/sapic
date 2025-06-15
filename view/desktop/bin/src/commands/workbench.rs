use anyhow::anyhow;
use moss_app::{app::App, context::AppContext};
use moss_tauri::{TauriError, TauriResult};
use moss_workbench::models::operations::*;
use std::time::Duration;
use tauri::{Runtime as TauriRuntime, State, Window};

use crate::constants::DEFAULT_COMMAND_TIMEOUT;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn create_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: CreateWorkspaceInput,
) -> TauriResult<CreateWorkspaceOutput> {
    // let r = tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {})
    //     .await
    //     .map_err(|err| anyhow!("Failed to create workspace: {}", err))?;

    let ctx = AppContext::from(&app);
    let workbench = app.workbench();
    let workspace_output = workbench
        .create_workspace(&ctx, &input)
        .await
        .map_err(|err| TauriError(format!("Failed to create workspace: {}", err)))?;

    Ok(workspace_output)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn list_workspaces<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<ListWorkspacesOutput> {
    let ctx = AppContext::from(&app);
    let workbench = app.workbench();
    let workspaces = workbench.list_workspaces(&ctx).await?;

    Ok(workspaces)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn delete_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: DeleteWorkspaceInput,
) -> TauriResult<()> {
    let ctx = AppContext::from(&app);
    let workbench = app.workbench();
    workbench.delete_workspace(&ctx, &input).await?;

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn open_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: OpenWorkspaceInput,
) -> TauriResult<OpenWorkspaceOutput> {
    let ctx = AppContext::from(&app);
    let workbench = app.workbench();
    let workspace_output = workbench.open_workspace(&ctx, &input).await?;

    Ok(workspace_output)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: UpdateWorkspaceInput,
) -> TauriResult<()> {
    let ctx = AppContext::from(&app);
    let workbench = app.workbench();
    workbench.update_workspace(&ctx, &input).await?;

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn describe_workbench_state<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<DescribeWorkbenchStateOutput> {
    let ctx = AppContext::from(&app);
    let workbench = app.workbench();
    let state = workbench.describe_state().await?;

    Ok(state)
}
