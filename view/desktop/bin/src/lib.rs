mod commands;
pub mod constants;
mod mem;
mod menu;
mod plugins;
mod services;
mod window;

#[macro_use]
extern crate tracing;

use moss_app::manager::AppManager;
use moss_fs::adapters::disk::DiskFileSystem;
use services::service_pool;
use std::sync::Arc;
use tauri::{AppHandle, Manager, RunEvent, WebviewWindow, WindowEvent};
use tauri_plugin_os;
use window::{create_window, CreateWindowInput};

use crate::constants::*;
use crate::plugins::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(plugin_log::init())
        .plugin(plugin_window_state::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}));

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(mac_window::init());
    }

    builder
        .setup(|app| {
            let fs = Arc::new(DiskFileSystem::new());
            let app_handle = app.app_handle();
            let service_pool = service_pool(app_handle, fs.clone());
            let app_manager = AppManager::new(app_handle.clone(), service_pool);
            app_handle.manage(app_manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::execute_command,
            commands::set_color_theme,
            commands::get_color_theme,
            commands::list_color_themes,
            commands::describe_app_state,
            commands::set_locale,
            commands::list_locales,
            commands::get_translations,
        ])
        .on_window_event(|window, event| match event {
            #[cfg(target_os = "macos")]
            WindowEvent::CloseRequested { api, .. } => {
                if window.app_handle().webview_windows().len() == 1 {
                    window.app_handle().hide().ok();
                    api.prevent_close();
                }
            }
            WindowEvent::Focused(_) => { /* call updates, git fetch, etc. */ }

            _ => (),
        })
        .build(tauri::generate_context!())
        .expect("failed to run")
        .run(|app_handle, event| match event {
            RunEvent::Ready => {
                let webview_window = create_main_window(&app_handle, "/");
                webview_window
                    .on_menu_event(move |window, event| menu::handle_event(window, &event));
            }

            #[cfg(target_os = "macos")]
            RunEvent::ExitRequested { api, .. } => {
                app_handle.hide().ok();
                api.prevent_exit();
            }

            _ => {}
        });
}

fn create_main_window(app_handle: &AppHandle, url: &str) -> WebviewWindow {
    // TODO: Use ConfigurationService

    let window_inner_height = DEFAULT_WINDOW_HEIGHT;
    let window_inner_width = DEFAULT_WINDOW_WIDTH;

    let label = format!("{MAIN_WINDOW_PREFIX}{}", 0);
    let config = CreateWindowInput {
        url,
        label: label.as_str(),
        title: "Sapic",
        inner_size: (window_inner_width, window_inner_height),
        position: (100.0, 100.0),
    };

    create_window(app_handle, config)
}
