use moss_app::{app::App, context::AppContext};
use moss_tauri::{TauriError, TauriResult};
use moss_workspace::models::{
    events::StreamEnvironmentsEvent,
    operations::{DescribeStateOutput, UpdateStateInput},
};
use tauri::{Runtime as TauriRuntime, State, Window, ipc::Channel as TauriChannel};

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_workspace_state<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    input: UpdateStateInput,
) -> TauriResult<()> {
    let _ctx = AppContext::from(&app);
    let workbench = app.workbench();
    let current_workspace = workbench
        .active_workspace()
        .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling
    current_workspace.update_state(input).await?;

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn describe_workspace_state<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
) -> TauriResult<DescribeStateOutput> {
    let _ctx = AppContext::from(&app);
    let workbench = app.workbench();
    let current_workspace = workbench
        .active_workspace()
        .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling
    let output = current_workspace.describe_state().await?;

    Ok(output)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_workspace_environments<R: TauriRuntime>(
    app: State<'_, App<R>>,
    window: Window<R>,
    channel: TauriChannel<StreamEnvironmentsEvent>,
) -> TauriResult<()> {
    let ctx = AppContext::from(&app);
    let workbench = app.workbench();
    let current_workspace = workbench
        .active_workspace()
        .ok_or_else(|| TauriError("No active workspace".to_string()))?; // TODO: improve error handling
    current_workspace.stream_environments(&ctx, channel).await?;

    Ok(())
}
