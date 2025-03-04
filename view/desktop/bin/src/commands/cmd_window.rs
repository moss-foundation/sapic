use anyhow::Result;
use moss_app::manager::AppManager;
use moss_tauri::TauriResult;
use moss_theme::{
    models::{events::ColorThemeChangeEventPayload, operations::ListThemesOutput},
    primitives::ThemeId,
    theme_service::ThemeService,
};
use tauri::{AppHandle, Emitter, EventTarget, Manager, State, Window};

use crate::{create_child_window, menu};

// According to https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindowBuilder.html
// We should call WebviewWindowBuilder from async commands
#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
pub async fn create_new_window(app_handle: AppHandle) -> TauriResult<()> {
    let webview_window = create_child_window(&app_handle, "/")?;
    webview_window.on_menu_event(move |window, event| menu::handle_event(window, &event));
    Ok(())
}

#[tauri::command]
#[instrument(level = "trace", skip(app_handle), fields(window = window.label()))]
pub fn change_color_theme(app_handle: AppHandle, window: Window, id: ThemeId) -> TauriResult<()> {
    for (label, _) in app_handle.webview_windows() {
        if window.label() == &label {
            continue;
        }

        app_handle
            .emit_to(
                EventTarget::webview_window(label),
                "core://color-theme-changed",
                ColorThemeChangeEventPayload::new(&id),
            )
            .unwrap();
    }

    Ok(())
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn read_color_theme(app_manager: State<'_, AppManager>, id: ThemeId) -> Result<String> {
    app_manager
        .service::<ThemeService>()?
        .read_color_theme(&id)
        .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn list_themes(app_manager: State<'_, AppManager>) -> Result<ListThemesOutput> {
    app_manager.service::<ThemeService>()?.list_themes().await
}
