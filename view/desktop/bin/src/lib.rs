mod commands;
pub mod constants;
mod mem;
mod menu;
mod plugins;
mod window;

#[macro_use]
extern crate tracing;

use anyhow::Result;
use moss_app::manager::AppManager;
use moss_app::service::InstantiationType;
use moss_fs::adapters::disk::DiskFileSystem;
use moss_nls::locale_service::LocaleService;
use moss_state::manager::AppStateManager;
use moss_tauri::services::window_service::WindowService;
use moss_tauri::TauriResult;
use moss_theme::theme_service::ThemeService;
use moss_workspace::workspace_manager::WorkspaceManager;
use rand::random;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager, RunEvent, WebviewWindow, WindowEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_os;
use window::{create_window, CreateWindowInput};

use crate::commands::*;
use crate::plugins::*;

pub use constants::*;
use moss_logging::{LogPayload, LogScope, LoggingService};
use moss_session::SessionService;
use moss_state::{
    command,
    command::{CommandContext, CommandDecl},
};
use moss_text::read_only_str;

async fn generate_log<'a>(
    ctx: &mut CommandContext,
    _manager: &'a AppStateManager,
) -> TauriResult<String> {
    ctx.app_handle()
        .state::<AppManager>()
        .service::<LoggingService>()
        .unwrap()
        .info(
            LogScope::App,
            LogPayload {
                collection: None,
                request: None,
                message: "Generate a log from the frontend".to_string(),
            },
        );

    Ok("Successfully generated a log!".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
            let app_handle = app.app_handle();
            let themes_dir: PathBuf = std::env::var("THEMES_DIR")
                .expect("Environment variable THEMES_DIR is not set")
                .into();
            let app_state = AppStateManager::new(&themes_dir).with_commands([
                // FIXME: Remove this example command
                command!("example.generateLog", generate_log),
            ]);
            app_handle.manage(app_state);

            let fs = Arc::new(DiskFileSystem::new());

            let session_service = SessionService::new();
            // FIXME: In the future, we will place logs at appropriate locations
            // Now we put `logs` folder at the project root for easier development
            let app_log_dir: PathBuf = std::env::var("APP_LOG_DIR")
                .expect("Environment variable APP_LOG_DIR is not set")
                .into();
            let session_log_dir: PathBuf = std::env::var("SESSION_LOG_DIR")
                .expect("Environment variable SESSION_LOG_DIR is not set")
                .into();
            let logging_service =
                LoggingService::new(&app_log_dir, &session_log_dir, &session_service)?;

            let app_manager = AppManager::new(app_handle.clone())
                .with_service(
                    {
                        let fs_clone = Arc::clone(&fs);
                        let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
                        let workspaces_dir: PathBuf =
                            PathBuf::from(dir).join("samples").join("workspaces");

                        move |_| WorkspaceManager::new(fs_clone, workspaces_dir)
                    },
                    InstantiationType::Instant,
                )
                .with_service(
                    {
                        let fs_clone = Arc::clone(&fs);
                        let locales_dir: PathBuf = std::env::var("LOCALES_DIR")
                            .expect("Environment variable LOCALES_DIR is not set")
                            .into();
                        move |_| LocaleService::new(fs_clone, locales_dir)
                    },
                    InstantiationType::Delayed,
                )
                .with_service(
                    {
                        let fs_clone = Arc::clone(&fs);

                        move |_| ThemeService::new(fs_clone, themes_dir)
                    },
                    InstantiationType::Delayed,
                )
                .with_service(|_| WindowService::new(), InstantiationType::Delayed)
                .with_service(|_| session_service, InstantiationType::Instant)
                .with_service(|_| logging_service, InstantiationType::Instant);
            app_handle.manage(app_manager);

            let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, shortcut, event| {
                        println!("{:?}", shortcut);
                        if shortcut == &ctrl_n_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    tauri::async_runtime::spawn(cmd_window::create_new_window(
                                        app.clone(),
                                    ));
                                }
                                ShortcutState::Released => {}
                            }
                        }
                    })
                    .build(),
            )?;
            app.global_shortcut().register(ctrl_n_shortcut)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::execute_command,
            commands::change_color_theme,
            commands::change_color_theme,
            commands::get_color_theme,
            commands::list_themes,
            commands::describe_app_state,
            commands::change_language_pack,
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

fn create_child_window(app_handle: &AppHandle, url: &str) -> Result<WebviewWindow> {
    let app_manager = app_handle.state::<AppManager>();
    let next_window_id = app_manager.service::<WindowService>()?.next_window_id() + 1;
    let config = CreateWindowInput {
        url,
        label: &format!("{MAIN_WINDOW_PREFIX}{}", next_window_id),
        title: "Sapic",
        inner_size: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
        position: (
            100.0 + random::<f64>() * 20.0,
            100.0 + random::<f64>() * 20.0,
        ),
    };

    Ok(create_window(app_handle, config))
}
