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
use moss_fs::RealFileSystem;
use services::service_pool;
use std::sync::Arc;
use tauri::{AppHandle, Manager, RunEvent, Runtime as TauriRuntime, WebviewWindow, WindowEvent};
use tauri::{Emitter, Listener};
use tauri_plugin_os;

use window::{create_window, CreateWindowInput};

use crate::constants::*;
use crate::plugins::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run<R: TauriRuntime>() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::<R>::new()
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
            let fs = Arc::new(RealFileSystem::new());
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
            commands::open_workspace,
            commands::set_layout_parts_state,
            commands::describe_layout_parts_state,
            commands::example_index_collection_command,
        ])
        .on_window_event(|window, event| match event {
            // #[cfg(target_os = "macos")]
            // WindowEvent::CloseRequested { api, .. } => {
            //     if window.app_handle().webview_windows().len() == 1 {
            //         window.app_handle().hide().ok();
            //         api.prevent_close();
            //     }
            // }
            WindowEvent::Focused(_) => { /* call updates, git fetch, etc. */ }
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();

                let app_handle = window.app_handle();
                app_handle.emit("kernel-windowCloseRequested", {}).unwrap();

                let window_clone = window.clone();
                app_handle.listen("kernel-windowCloseRequestedConfirmed", move |_event| {
                    window_clone.close().ok();
                });
            }

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

            RunEvent::Exit => {
                dbg!("Exit");
            }

            // #[cfg(target_os = "macos")]
            // RunEvent::ExitRequested { api, .. } => {
            //     dbg!("ExitRequested");

            //     // api.prevent_exit();
            //     // app_handle.hide().ok();

            //     // FIXME: Temporary solution
            //     app_handle.emit("kernel-windowCloseRequested", {}).unwrap();

            //     let app_handle_clone = app_handle.clone();
            //     app_handle.listen("kernel-windowCloseRequestedConfirmed", move |_event| {
            //         #[cfg(target_os = "macos")]
            //         {
            //             app_handle_clone.hide().ok();
            //         }
            //     });
            // }
            _ => {}
        });
}

fn create_main_window<R: TauriRuntime>(app_handle: &AppHandle<R>, url: &str) -> WebviewWindow<R> {
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
