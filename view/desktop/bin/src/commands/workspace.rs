use moss_app::manager::AppManager;
use moss_tauri::{TauriError, TauriResult};
use moss_workbench::workbench::Workbench;
use moss_workspace::models::{
    events::StreamEnvironmentsEvent,
    operations::{DescribeStateOutput, UpdateStateInput},
};
use tauri::{Runtime as TauriRuntime, State, Window, ipc::Channel as TauriChannel};

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label()))]
pub async fn update_workspace_state<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: UpdateStateInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let workbench = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    let current_workspace = workbench
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
    let workbench = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    let current_workspace = workbench
        .active_workspace()
        .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling

    let output = current_workspace
        .describe_state()
        .await
        .map_err(|err| TauriError(format!("Failed to describe layout parts state: {}", err)))?;

    Ok(output)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_workspace_environments<R: TauriRuntime>(
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    channel: TauriChannel<StreamEnvironmentsEvent>,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let workbench = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    let current_workspace = workbench
        .active_workspace()
        .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling

    current_workspace.stream_environments(channel).await?;

    Ok(())
}
