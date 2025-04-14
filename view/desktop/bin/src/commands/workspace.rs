use moss_app::manager::AppManager;
use moss_tauri::{TauriError, TauriResult};
use moss_workspace::{
    models::operations::{
        DescribeLayoutPartsStateOutput, OpenWorkspaceInput, SetLayoutPartsStateInput,
        SetLayoutPartsStateParams,
    },
    workspace_manager::WorkspaceManager,
};
use tauri::{Runtime as TauriRuntime, State, Window};

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn open_workspace<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: OpenWorkspaceInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<WorkspaceManager<R>>(&app_handle)
        .await?;

    workspace_manager
        .open_workspace(&input)
        .await
        .map_err(|err| TauriError(format!("Failed to open workspace: {}", err)))?;

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn set_layout_parts_state<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: SetLayoutPartsStateInput,
    params: SetLayoutPartsStateParams,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<WorkspaceManager<R>>(&app_handle)
        .await?;

    let current_workspace = workspace_manager.current_workspace()?;
    current_workspace
        .1
        .set_layout_parts_state(input, params)
        .await
        .map_err(|err| TauriError(format!("Failed to set layout parts state: {}", err)))?;

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn describe_layout_parts_state<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
) -> TauriResult<DescribeLayoutPartsStateOutput> {
    let app_handle = app_manager.app_handle();
    let workspace_manager = app_manager
        .services()
        .get_by_type::<WorkspaceManager<R>>(&app_handle)
        .await?;

    let current_workspace = workspace_manager.current_workspace()?;
    let output = current_workspace
        .1
        .describe_layout_parts_state()
        .await
        .map_err(|err| TauriError(format!("Failed to describe layout parts state: {}", err)))?;

    Ok(output)
}
