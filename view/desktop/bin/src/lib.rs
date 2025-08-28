mod commands;
mod constants;
mod mem;
mod menu;
mod plugins;
mod window;

#[macro_use]
extern crate tracing;

use moss_app::{AppBuilder, builder::BuildAppParams};
use moss_applib::{
    TauriAppRuntime,
    context::{AnyAsyncContext, AnyContext, MutableContext},
};
use moss_fs::RealFileSystem;
use std::{path::PathBuf, sync::Arc, time::Duration};
#[cfg(not(debug_assertions))]
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, RunEvent, Runtime as TauriRuntime, WebviewWindow, WindowEvent};
use tauri_plugin_os;
use window::{CreateWindowInput, create_window};

use crate::{constants::*, plugins::*};

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
        .setup(|tao| {
            futures::executor::block_on(async {
                let ctx = MutableContext::background().freeze();

                let fs = Arc::new(RealFileSystem::new());
                let app_handle = tao.app_handle();

                #[cfg(debug_assertions)]
                let (app_dir, themes_dir, locales_dir, logs_dir) = {
                    (
                        PathBuf::from(
                            std::env::var("DEV_APP_DIR").expect("DEV_APP_DIR is not set"),
                        ),
                        PathBuf::from(
                            std::env::var("THEMES_DIR")
                                .expect("Environment variable THEMES_DIR is not set"),
                        ),
                        PathBuf::from(
                            std::env::var("LOCALES_DIR")
                                .expect("Environment variable LOCALES_DIR is not set"),
                        ),
                        PathBuf::from(
                            std::env::var("APP_LOG_DIR")
                                .expect("Environment variable APP_LOG_DIR is not set"),
                        ),
                    )
                };

                #[cfg(not(debug_assertions))]
                let (app_dir, themes_dir, locales_dir, logs_dir) = {
                    let paths = tao.path();
                    (
                        paths.app_data_dir().expect("cannot resolve app data dir"),
                        paths
                            .resolve("resources/themes", tauri::path::BaseDirectory::Resource)
                            .expect("cannot resolve themes dir"),
                        paths
                            .resolve("resources/locales", tauri::path::BaseDirectory::Resource)
                            .expect("cannot resolve locales dir"),
                        paths.app_log_dir().expect("cannot resolve app log dir"),
                    )
                };

                let ctx_clone = ctx.clone();
                let (app, session_id) = {
                    let app_init_ctx =
                        MutableContext::new_with_timeout(ctx_clone, Duration::from_secs(30))
                            .freeze();

                    let app = AppBuilder::<TauriAppRuntime<R>>::new(app_handle.clone(), fs)
                        .build(
                            &app_init_ctx,
                            BuildAppParams {
                                app_dir,
                                themes_dir,
                                locales_dir,
                                logs_dir,
                            },
                        )
                        .await;
                    let session_id = app.session_id().clone();

                    (app, session_id)
                };

                app_handle.manage({
                    let mut ctx = ctx.unfreeze().expect("Failed to unfreeze the root context");
                    ctx.with_value("session_id", session_id.to_string()); // TODO: Use a proper type

                    ctx.freeze()
                });
                app_handle.manage(app);

                Ok(())
            })
        })
        .invoke_handler(tauri::generate_handler![
            //
            // App
            //
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
            commands::create_workspace,
            commands::list_workspaces,
            commands::delete_workspace,
            commands::close_workspace,
            commands::cancel_request,
            commands::add_account,
            //
            // Workspace
            //
            commands::stream_environments,
            commands::update_workspace_state,
            commands::describe_workspace_state,
            commands::stream_collections,
            commands::describe_collection,
            commands::create_collection,
            commands::import_collection,
            commands::delete_collection,
            commands::update_collection,
            commands::batch_update_collection,
            commands::activate_environment,
            commands::create_environment,
            commands::update_environment,
            commands::stream_environments,
            commands::delete_environment,
            commands::update_environment_group,
            commands::batch_update_environment_group,
            //
            // Collection
            //
            commands::create_collection_entry,
            commands::delete_collection_entry,
            commands::stream_collection_entries,
            commands::update_collection_entry,
            commands::batch_update_collection_entry,
            commands::batch_create_collection_entry,
            //
            // Env
            //
            commands::get_mistral_api_key,
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

            RunEvent::Exit => {}

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
