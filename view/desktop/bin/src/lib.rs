mod commands;
mod constants;
mod mem;
mod menu;
mod plugins;
mod window;

#[macro_use]
extern crate tracing;

use moss_activity_indicator::ActivityIndicator;
use moss_app::{
    AppBuilder,
    app::AppDefaults,
    services::{
        locale_service::LocaleService, log_service::LogService, session_service::SessionService,
        storage_service::StorageService, theme_service::ThemeService,
        workspace_service::WorkspaceService,
    },
};
use moss_applib::{
    TauriAppRuntime,
    context::{AnyAsyncContext, AnyContext, MutableContext},
    context_old::ContextValueSet,
};
use moss_environment::Environment;
use moss_fs::{RealFileSystem, model_registry::GlobalModelRegistry};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tauri::{AppHandle, Manager, RunEvent, Runtime as TauriRuntime, WebviewWindow, WindowEvent};
use tauri_plugin_os;
use window::{CreateWindowInput, create_window};

use crate::{constants::*, plugins::*};

type GlobalEnvironmentRegistry<R> = moss_environment::GlobalEnvironmentRegistry<
    TauriAppRuntime<R>,
    Environment<TauriAppRuntime<R>>,
>;

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

                let app_dir =
                    PathBuf::from(std::env::var("DEV_APP_DIR").expect("DEV_APP_DIR is not set"));

                let themes_dir: PathBuf = std::env::var("THEMES_DIR")
                    .expect("Environment variable THEMES_DIR is not set")
                    .into();

                let locales_dir: PathBuf = std::env::var("LOCALES_DIR")
                    .expect("Environment variable LOCALES_DIR is not set")
                    .into();

                let logs_dir: PathBuf = std::env::var("APP_LOG_DIR")
                    .expect("Environment variable APP_LOG_DIR is not set")
                    .into();

                let ctx_clone = ctx.clone();
                let (app, session_id) = {
                    let app_init_ctx =
                        MutableContext::new_with_timeout(ctx_clone, Duration::from_secs(30))
                            .freeze();

                    let storage_service: Arc<StorageService<TauriAppRuntime<R>>> =
                        StorageService::<TauriAppRuntime<R>>::new(
                            &app_dir.join(moss_app::dirs::GLOBALS_DIR),
                        )
                        .expect("Failed to create storage service")
                        .into();

                    let session_service = SessionService::new();
                    let session_id = session_service.session_id().clone();

                    let environment_registry = GlobalEnvironmentRegistry::<R>::new();
                    let model_registry = GlobalModelRegistry::new();

                    let workspace_service = WorkspaceService::<TauriAppRuntime<R>>::new(
                        &app_init_ctx,
                        storage_service.clone(),
                        fs.clone(),
                        &app_dir,
                        Arc::new(environment_registry),
                        Arc::new(model_registry),
                    )
                    .await
                    .expect("Failed to create workspace service");
                    let theme_service = ThemeService::new(fs.clone(), themes_dir);
                    let locale_service = LocaleService::new(fs.clone(), locales_dir);
                    let log_service = LogService::new(
                        fs.clone(),
                        app_handle.clone(),
                        &logs_dir,
                        session_service.session_id(),
                        storage_service.clone(),
                    )
                    .expect("Failed to create log service");

                    let default_theme = theme_service
                        .default_theme()
                        .await
                        .cloned()
                        .expect("Failed to get default theme");

                    let default_locale = locale_service
                        .default_locale()
                        .await
                        .cloned()
                        .expect("Failed to get default locale");

                    let defaults = AppDefaults {
                        theme: default_theme,
                        locale: default_locale,
                    };

                    let activity_indicator = ActivityIndicator::new(app_handle.clone());
                    let app = AppBuilder::<TauriAppRuntime<R>>::new(
                        app_handle.clone(),
                        activity_indicator,
                        defaults,
                        fs,
                        app_dir.into(),
                    )
                    .with_global(GlobalModelRegistry::new())
                    .with_global(GlobalEnvironmentRegistry::<R>::new())
                    .with_service::<StorageService<TauriAppRuntime<R>>>(storage_service)
                    .with_service(theme_service)
                    .with_service(locale_service)
                    .with_service(session_service)
                    .with_service(log_service)
                    .with_service(workspace_service)
                    .build()
                    .await
                    .expect("Failed to build the app");

                    (app, session_id)
                };

                app_handle.manage({
                    let mut ctx = ctx.unfreeze().expect("Failed to unfreeze the root context");
                    ctx.with_value("session_id", session_id.to_string()); // TODO: Use a proper type

                    ctx.freeze()
                });
                app_handle.manage(app);
                app_handle.manage(ContextValueSet::default());

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
            //
            // Workspace
            //
            commands::stream_environments,
            commands::update_workspace_state,
            commands::describe_workspace_state,
            commands::stream_collections,
            commands::create_collection,
            commands::delete_collection,
            commands::update_collection,
            commands::batch_update_collection,
            commands::create_environment,
            commands::update_environment,
            commands::stream_environments,
            //
            // Collection
            //
            commands::create_collection_entry,
            commands::delete_collection_entry,
            commands::stream_collection_entries,
            commands::update_collection_entry,
            commands::batch_update_collection_entry,
            commands::batch_create_collection_entry,
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
