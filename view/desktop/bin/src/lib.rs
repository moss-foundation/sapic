mod commands;
mod constants;
mod mem;
mod menu;
mod plugins;
mod primitives;
mod services;
mod window;

#[macro_use]
extern crate tracing;

use moss_app::{context::AppContextBuilder, manager::AppManager};
use moss_fs::{FileSystem, RealFileSystem};
use moss_storage::global_storage::GlobalStorageImpl;
use moss_workbench::workbench::{Options as WorkbenchOptions, Workbench};
use services::service_pool;
use std::{path::PathBuf, sync::Arc};
use tauri::{AppHandle, Manager, RunEvent, Runtime as TauriRuntime, WebviewWindow, WindowEvent};
use tauri_plugin_os;

use window::{CreateWindowInput, create_window};

use crate::{commands::StateContext, constants::*, plugins::*};

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

            let app_dir =
                PathBuf::from(std::env::var("DEV_APP_DIR").expect("DEV_APP_DIR is not set"));

            let global_storage = Arc::new(
                GlobalStorageImpl::new(&app_dir).expect("Failed to create global storage"),
            );

            let workbench = Workbench::new(
                app_handle.clone(),
                global_storage,
                WorkbenchOptions {
                    abs_path: app_dir.clone().into(),
                },
            );

            app_handle.manage(workbench);

            let service_pool = service_pool(&app_handle, fs.clone());
            let app_manager = AppManager::new(app_handle.clone(), service_pool);
            app_handle.manage(app_manager);

            let mut context_builder = AppContextBuilder::new();
            <dyn FileSystem>::set_global(fs, &mut context_builder);
            let ctx = context_builder.build(app_handle.clone());

            app_handle.manage(StateContext::from(ctx));

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
            commands::update_workspace,
            commands::update_workspace_state,
            commands::describe_workspace_state,
            commands::create_workspace,
            commands::list_workspaces,
            commands::delete_workspace,
            commands::describe_workbench_state,
            commands::stream_workspace_environments,
        ])
        .on_window_event(|_window, event| match event {
            // #[cfg(target_os = "macos")]
            // WindowEvent::CloseRequested { api, .. } => {
            //     if window.app_handle().webview_windows().len() == 1 {
            //         window.app_handle().hide().ok();
            //         api.prevent_close();
            //     }
            // }
            WindowEvent::Focused(_) => { /* call updates, git fetch, etc. */ }
            WindowEvent::CloseRequested { .. } => {}

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
