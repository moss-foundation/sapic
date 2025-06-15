use moss_app::{app::App, context::AppContext};
use moss_tauri::{TauriError, TauriResult};
use moss_workbench::models::operations::*;
use tauri::{Runtime as TauriRuntime, State, Window};

use crate::constants::DEFAULT_COMMAND_TIMEOUT;

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn create_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: CreateWorkspaceInput,
) -> TauriResult<CreateWorkspaceOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        workbench
            .create_workspace(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn list_workspaces<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<ListWorkspacesOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        workbench
            .list_workspaces(&ctx)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn delete_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: DeleteWorkspaceInput,
) -> TauriResult<()> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        workbench
            .delete_workspace(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn open_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: OpenWorkspaceInput,
) -> TauriResult<OpenWorkspaceOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        workbench
            .open_workspace(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_workspace<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: UpdateWorkspaceInput,
) -> TauriResult<()> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        workbench
            .update_workspace(&ctx, &input)
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn describe_workbench_state<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<DescribeWorkbenchStateOutput> {
    tokio::time::timeout(DEFAULT_COMMAND_TIMEOUT, async move {
        let ctx = AppContext::from(&app);
        let workbench = app.workbench();
        workbench
            .describe_state()
            .await
            .map_err(TauriError::OperationError)
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}
