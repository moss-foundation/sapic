use tauri::{AppHandle, State};
use moss_desktop::app::state::AppStateManager;
use moss_desktop::models::application::{AppState, Defaults, Preferences};
use moss_tauri::TauriResult;
use crate::{create_child_window, menu};

// According to https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindowBuilder.html
// We should call WebviewWindowBuilder from async commands
#[tauri::command]
#[instrument(level = "trace", skip(app_handle))]
pub async fn create_new_window(app_handle: AppHandle) -> TauriResult<()> {
    let webview_window = create_child_window(&app_handle, "/")?;
    webview_window.on_menu_event(move |window, event| menu::handle_event(window, &event));
    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state_manager))]
pub fn get_state(state_manager: State<'_, AppStateManager>) -> Result<AppState, String> {
    // TODO: AppState
    Ok(AppState {
        preferences: Preferences {},
        defaults: Defaults {},
    })
}