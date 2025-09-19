mod commands;
mod constants;
mod mem;
mod menu;
mod plugins;
mod window;

#[macro_use]
extern crate tracing;

use moss_app::{
    App, AppBuilder as TauriAppBuilder, app::OnAppReadyOptions, builder::BuildAppParams,
};
use moss_app_delegate::AppDelegate;
use moss_applib::{
    TauriAppRuntime,
    context::{AnyAsyncContext, AnyContext, MutableContext},
};
use moss_fs::RealFileSystem;
use moss_git_hosting_provider::{
    github::{
        RealGitHubApiClient, RealGitHubAuthAdapter, auth::GitHubAuthAdapter,
        client::GitHubApiClient,
    },
    gitlab::{
        RealGitLabApiClient, RealGitLabAuthAdapter, auth::GitLabAuthAdapter,
        client::GitLabApiClient,
    },
};
use moss_keyring::KeyringClientImpl;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use reqwest::ClientBuilder as HttpClientBuilder;
use std::{path::PathBuf, sync::Arc, time::Duration};
#[cfg(not(debug_assertions))]
use tauri::path::BaseDirectory;
use tauri::{
    AppHandle as TauriAppHandle, Manager, RunEvent, Runtime as TauriRuntime, WebviewWindow,
    WindowEvent,
};
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

                let keyring = Arc::new(KeyringClientImpl::new());
                let http_client = HttpClientBuilder::new()
                    .user_agent("SAPIC/1.0")
                    .build()
                    .expect("failed to build http client");
                let auth_api_client = Arc::new(AccountAuthGatewayApiClient::new(
                    http_client.clone(),
                    dotenv::var("ACCOUNT_AUTH_BASE_URL").expect("ACCOUNT_AUTH_BASE_URL is not set"),
                ));

                let tao_app_handle = tao.app_handle();

                #[cfg(debug_assertions)]
                let (themes_dir, locales_dir, logs_dir, temp_dir) = {
                    (
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
                        PathBuf::from(
                            std::env::var("TEMP_DIR")
                                .expect("Environment variable TEMP_DIR is not set"),
                        ),
                    )
                };

                #[cfg(not(debug_assertions))]
                let (themes_dir, locales_dir, logs_dir, temp_dir) = {
                    let paths = tao.path();
                    (
                        paths
                            .resolve("resources/themes", tauri::path::BaseDirectory::Resource)
                            .expect("cannot resolve themes dir"),
                        paths
                            .resolve("resources/locales", tauri::path::BaseDirectory::Resource)
                            .expect("cannot resolve locales dir"),
                        paths.app_log_dir().expect("cannot resolve app log dir"),
                        paths.temp_dir().expect("cannot resolve temp dir"),
                    )
                };
                let fs = Arc::new(RealFileSystem::new(&temp_dir));

                // Registration of global resources that will be accessible
                // throughout the entire application via the `global` method
                // of the app's internal handler.

                {
                    let delegate = AppDelegate::<TauriAppRuntime<R>>::new(tao_app_handle.clone());

                    <dyn GitHubApiClient<TauriAppRuntime<R>>>::set_global(
                        &delegate,
                        Arc::new(RealGitHubApiClient::new(http_client.clone())),
                    );
                    <dyn GitHubAuthAdapter<TauriAppRuntime<R>>>::set_global(
                        &delegate,
                        Arc::new(RealGitHubAuthAdapter::<TauriAppRuntime<R>>::new(
                            auth_api_client.clone(),
                            auth_api_client.base_url(),
                            8080,
                        )),
                    );

                    <dyn GitLabApiClient<TauriAppRuntime<R>>>::set_global(
                        &delegate,
                        Arc::new(RealGitLabApiClient::new(http_client.clone())),
                    );
                    <dyn GitLabAuthAdapter<TauriAppRuntime<R>>>::set_global(
                        &delegate,
                        Arc::new(RealGitLabAuthAdapter::<TauriAppRuntime<R>>::new(
                            auth_api_client.clone(),
                            auth_api_client.base_url(),
                            8081,
                        )),
                    );

                    tao_app_handle.manage(delegate);
                }

                let ctx_clone = ctx.clone();
                let (app, session_id) = {
                    let app_init_ctx =
                        MutableContext::new_with_timeout(ctx_clone, Duration::from_secs(30))
                            .freeze();

                    let app = TauriAppBuilder::<TauriAppRuntime<R>>::new(
                        tao_app_handle.clone(),
                        fs,
                        keyring,
                        auth_api_client,
                    )
                    .build(
                        &app_init_ctx,
                        BuildAppParams {
                            themes_dir,
                            locales_dir,
                            logs_dir,
                        },
                    )
                    .await;
                    let session_id = app.session_id().clone();

                    (app, session_id)
                };

                tao_app_handle.manage({
                    let mut ctx = ctx.unfreeze().expect("Failed to unfreeze the root context");
                    ctx.with_value("session_id", session_id.to_string()); // TODO: Use a proper type

                    ctx.freeze()
                });
                tao_app_handle.manage(Arc::new(app));

                Ok(())
            })
        })
        .invoke_handler(tauri::generate_handler![
            //
            // App
            //
            commands::describe_app,
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
            commands::update_profile,
            //
            // Workspace
            //
            commands::stream_environments,
            commands::update_workspace_state,
            commands::describe_workspace_state,
            commands::stream_projects,
            commands::describe_project,
            commands::create_project,
            commands::import_project,
            commands::export_project,
            commands::delete_project,
            commands::update_project,
            commands::archive_project,
            commands::unarchive_project,
            commands::batch_update_project,
            commands::list_changes,
            commands::activate_environment,
            commands::create_environment,
            commands::update_environment,
            commands::batch_update_environment,
            commands::stream_environments,
            commands::delete_environment,
            commands::update_environment_group,
            commands::batch_update_environment_group,
            //
            // Project
            //
            commands::create_project_entry,
            commands::delete_project_entry,
            commands::stream_project_entries,
            commands::update_project_entry,
            commands::describe_project_entry,
            commands::batch_update_project_entry,
            commands::batch_create_project_entry,
            commands::execute_vcs_operation,
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

                futures::executor::block_on(async {
                    let app = app_handle.state::<Arc<App<TauriAppRuntime<R>>>>();
                    let ctx =
                        MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();
                    let app_delegate = app_handle
                        .state::<AppDelegate<TauriAppRuntime<R>>>()
                        .inner()
                        .clone();

                    app.on_app_ready(
                        &ctx,
                        &app_delegate,
                        OnAppReadyOptions {
                            restore_last_workspace: false,
                        },
                    )
                    .await
                    .expect("Failed to prepare the app");
                });
            }

            RunEvent::Exit => {}

            _ => {}
        });
}

fn create_main_window<R: TauriRuntime>(
    app_handle: &TauriAppHandle<R>,
    url: &str,
) -> WebviewWindow<R> {
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
