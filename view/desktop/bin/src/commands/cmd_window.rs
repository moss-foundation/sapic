use crate::{create_child_window, menu};
use moss_tauri::TauriResult;
use tauri::{AppHandle, State};

// According to https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindowBuilder.html
// We should call WebviewWindowBuilder from async commands
#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
pub async fn create_new_window(app_handle: AppHandle) -> TauriResult<()> {
    let webview_window = create_child_window(&app_handle, "/")?;
    webview_window.on_menu_event(move |window, event| menu::handle_event(window, &event));
    Ok(())
}
