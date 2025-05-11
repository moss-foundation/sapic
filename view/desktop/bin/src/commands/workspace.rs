use moss_app::manager::AppManager;
use moss_tauri::{TauriError, TauriResult};
use moss_workbench::{models::operations::*, workbench::Workbench};
use moss_workspace::models::operations::{DescribeStateOutput, UpdateStateInput};
use tauri::{Runtime as TauriRuntime, State, Window};

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn create_workspace<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: CreateWorkspaceInput,
) -> TauriResult<CreateWorkspaceOutput> {
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    let workspace_output = workspace_manager
        .create_workspace(&input)
        .await
        .map_err(|err| TauriError(format!("Failed to create workspace: {}", err)))?; // TODO: improve error handling

    Ok(workspace_output)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn list_workspaces<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
) -> TauriResult<ListWorkspacesOutput> {
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    workspace_manager
        .list_workspaces()
        .await
        .map_err(|err| TauriError(format!("Failed to list workspaces: {}", err))) // TODO: improve error handling
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn delete_workspace<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: DeleteWorkspaceInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    workspace_manager
        .delete_workspace(&input)
        .await
        .map_err(|err| TauriError(format!("Failed to delete workspace: {}", err))) // TODO: improve error handling
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn open_workspace<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: OpenWorkspaceInput,
) -> TauriResult<OpenWorkspaceOutput> {
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    workspace_manager
        .open_workspace(&input)
        .await
        .map_err(|err| TauriError(format!("Failed to open workspace: {}", err))) // TODO: improve error handling
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn update_workspace_state<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: UpdateStateInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    let current_workspace = workspace_manager
        .active_workspace()
        .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling
    current_workspace.update_state(input).await?;

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn describe_workspace_state<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
) -> TauriResult<DescribeStateOutput> {
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    let current_workspace = workspace_manager
        .active_workspace()
        .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling

    let output = current_workspace
        .describe_state()
        .await
        .map_err(|err| TauriError(format!("Failed to describe layout parts state: {}", err)))?;

    Ok(output)
}
