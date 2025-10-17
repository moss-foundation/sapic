mod commands;
mod constants;
mod mem;
mod menu;
mod plugins;
mod window;

#[macro_use]
extern crate tracing;

use moss_app::{App, AppBuilder as TauriAppBuilder, app::OnAppReadyOptions, command::CommandDecl};
use moss_app_delegate::AppDelegate;
use moss_applib::{
    TauriAppRuntime,
    context::{AnyAsyncContext, AnyContext, MutableContext},
};
use moss_configuration::registry::{AppConfigurationRegistry, ConfigurationRegistry};
use moss_extension_points::{
    configurations::ConfigurationExtensionPoint, http_headers::HttpHeadersExtensionPoint,
    languages::LanguageExtensionPoint, resource_statuses::ResourceStatusesExtensionPoint,
    themes::ThemeExtensionPoint,
};
use moss_fs::RealFileSystem;
use moss_git_hosting_provider::{
    github::{
        AppGitHubApiClient, AppGitHubAuthAdapter, auth::GitHubAuthAdapter, client::GitHubApiClient,
    },
    gitlab::{
        AppGitLabApiClient, AppGitLabAuthAdapter, auth::GitLabAuthAdapter, client::GitLabApiClient,
    },
};
use moss_keyring::KeyringClientImpl;
use moss_language::{
    RegisterTranslationContribution,
    registry::{AppLanguageRegistry, LanguageRegistry},
};
use moss_project::registries::{
    http_headers::{AppHttpHeaderRegistry, HttpHeaderRegistry},
    resource_statuses::{AppResourceStatusRegistry, ResourceStatusRegistry},
};
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_theme::registry::{AppThemeRegistry, ThemeRegistry};
use reqwest::ClientBuilder as HttpClientBuilder;
use serde_json::Value;
use std::{sync::Arc, time::Duration};
#[cfg(not(debug_assertions))]
use tauri::path::BaseDirectory;
use tauri::{
    AppHandle as TauriAppHandle, Emitter, Manager, RunEvent, Runtime as TauriRuntime,
    WebviewWindow, WindowEvent,
};
use tauri_plugin_os;
use window::{CreateWindowInput, create_window};

use crate::{constants::*, plugins::*};

inventory::submit! {
    RegisterTranslationContribution(include_str!(concat!(env!("OUT_DIR"), "/main.i18n.json")))
}

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

                let delegate = AppDelegate::<TauriAppRuntime<R>>::new(tao_app_handle.clone());
                let fs = Arc::new(RealFileSystem::new(&delegate.tmp_dir()));

                // Registration of global resources that will be accessible
                // throughout the entire application via the `global` method
                // of the app's internal handler.
                {
                    let github_api_client = Arc::new(AppGitHubApiClient::new(http_client.clone()));
                    let github_auth_adapter =
                        Arc::new(AppGitHubAuthAdapter::<TauriAppRuntime<R>>::new(
                            auth_api_client.clone(),
                            auth_api_client.base_url(),
                            8080,
                        ));
                    let gitlab_api_client = Arc::new(AppGitLabApiClient::new(http_client.clone()));
                    let gitlab_auth_adapter =
                        Arc::new(AppGitLabAuthAdapter::<TauriAppRuntime<R>>::new(
                            auth_api_client.clone(),
                            auth_api_client.base_url(),
                            8081,
                        ));

                    <dyn GitHubApiClient<TauriAppRuntime<R>>>::set_global(
                        &delegate,
                        github_api_client,
                    );
                    <dyn GitHubAuthAdapter<TauriAppRuntime<R>>>::set_global(
                        &delegate,
                        github_auth_adapter,
                    );

                    <dyn GitLabApiClient<TauriAppRuntime<R>>>::set_global(
                        &delegate,
                        gitlab_api_client,
                    );
                    <dyn GitLabAuthAdapter<TauriAppRuntime<R>>>::set_global(
                        &delegate,
                        gitlab_auth_adapter,
                    );

                    let theme_registry = AppThemeRegistry::new();
                    let locale_registry = AppLanguageRegistry::new();
                    let configuration_registry = AppConfigurationRegistry::new()
                        .expect("failed to build configuration registry");
                    let resource_status_registry = AppResourceStatusRegistry::new()
                        .expect("failed to build resource status registry");
                    let http_header_registry =
                        AppHttpHeaderRegistry::new().expect("failed to build http header registry");

                    <dyn ThemeRegistry>::set_global(&delegate, theme_registry);
                    <dyn LanguageRegistry>::set_global(&delegate, locale_registry);
                    <dyn ConfigurationRegistry>::set_global(&delegate, configuration_registry);
                    <dyn ResourceStatusRegistry>::set_global(&delegate, resource_status_registry);
                    <dyn HttpHeaderRegistry>::set_global(&delegate, http_header_registry);

                    tao_app_handle.manage(delegate);
                }

                let ctx_clone = ctx.clone();
                let (app, session_id) = {
                    let shortcut_println_command =
                        CommandDecl::<R>::new("shortcut.println".into(), |_ctx| {
                            Box::pin(async move {
                                println!("Triggering println using shortcut");
                                Ok(Value::Null)
                            })
                        });

                    let shortcut_alert_command =
                        CommandDecl::<R>::new("shortcut.alert".into(), |ctx| {
                            Box::pin(async move {
                                let _ = ctx.window().emit("alert", ());
                                Ok(Value::Null)
                            })
                        });

                    let app_init_ctx =
                        MutableContext::new_with_timeout(ctx_clone, Duration::from_secs(30))
                            .freeze();

                    let app = TauriAppBuilder::<TauriAppRuntime<R>>::new(
                        tao_app_handle.clone(),
                        fs,
                        keyring,
                        auth_api_client,
                        vec![
                            ThemeExtensionPoint::new(),
                            LanguageExtensionPoint::new(),
                            ConfigurationExtensionPoint::new(),
                            ResourceStatusesExtensionPoint::new(),
                            HttpHeadersExtensionPoint::new(),
                        ],
                    )
                    .with_command(shortcut_println_command)
                    .with_command(shortcut_alert_command)
                    .build(&app_init_ctx)
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
            commands::update_configuration,
            commands::list_configuration_schemas,
            commands::execute_command,
            commands::describe_color_theme,
            commands::list_color_themes,
            commands::list_locales,
            commands::get_translation_namespace,
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
            commands::update_layout,
            commands::describe_workspace,
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
                            restore_last_workspace: true,
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
