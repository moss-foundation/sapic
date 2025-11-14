mod commands;
mod constants;
mod mem;
mod menu;
mod plugins;

#[macro_use]
extern crate tracing;

use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{
    TauriAppRuntime,
    context::{AnyAsyncContext, MutableContext},
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
use moss_server_api::{
    account_auth_gateway::AccountAuthGatewayApiClient,
    extension_registry::{AppExtensionRegistryApiClient, ExtensionRegistryApiClient},
};
use moss_storage2::{AppStorage, Storage};
use reqwest::ClientBuilder as HttpClientBuilder;
use sapic_app::{builder::AppBuilder, command::CommandDecl};
use sapic_runtime::globals::GlobalThemeRegistry;
use sapic_system::theme::theme_registry::AppThemeRegistry;
use serde_json::Value;
use std::{sync::Arc, time::Duration};
#[cfg(not(debug_assertions))]
use tauri::path::BaseDirectory;
use tauri::{Emitter, Manager, RunEvent, Runtime as TauriRuntime, WindowEvent};
use tauri_plugin_os;

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
        // .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .plugin(shared_storage::init(|app| {
            let handle = app
                .downcast::<R>()
                .ok_or_join_err::<()>("failed to downcast app handle")?;

            let delegate = handle
                .state::<AppDelegate<TauriAppRuntime<R>>>()
                .inner()
                .clone();

            Ok(<dyn Storage>::global(&delegate))
        }));

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

                let server_api_endpoint =
                    dotenvy::var("SERVER_API_ENDPOINT").expect("SERVER_API_ENDPOINT is not set");

                let auth_api_client = Arc::new(AccountAuthGatewayApiClient::new(
                    http_client.clone(),
                    format!("{server_api_endpoint}/account-auth-gateway"),
                ));

                let tao_app_handle = tao.app_handle();

                let delegate = AppDelegate::<TauriAppRuntime<R>>::new(tao_app_handle.clone());
                let fs = Arc::new(RealFileSystem::new(&delegate.tmp_dir()));

                // Registration of global resources that will be accessible
                // throughout the entire application via the `global` method
                // of the app's internal handler.
                {
                    let storage =
                        AppStorage::new(&delegate.globals_dir(), delegate.workspaces_dir(), None)
                            .await
                            .expect("failed to create storage");
                    <dyn Storage>::set_global(&delegate, storage);

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

                    let extension_registry_api_client =
                        Arc::new(AppExtensionRegistryApiClient::new(
                            http_client.clone(),
                            format!("{server_api_endpoint}/extension-registry"),
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

                    <dyn ExtensionRegistryApiClient<TauriAppRuntime<R>>>::set_global(
                        &delegate,
                        extension_registry_api_client,
                    );

                    let theme_registry = AppThemeRegistry::new();
                    let languages_registry = AppLanguageRegistry::new();
                    let configuration_registry = AppConfigurationRegistry::new()
                        .expect("failed to build configuration registry");
                    let resource_status_registry = AppResourceStatusRegistry::new()
                        .expect("failed to build resource status registry");
                    let http_header_registry =
                        AppHttpHeaderRegistry::new().expect("failed to build http header registry");

                    GlobalThemeRegistry::set(&delegate, theme_registry);
                    <dyn LanguageRegistry>::set_global(&delegate, languages_registry);
                    <dyn ConfigurationRegistry>::set_global(&delegate, configuration_registry);
                    <dyn ResourceStatusRegistry>::set_global(&delegate, resource_status_registry);
                    <dyn HttpHeaderRegistry>::set_global(&delegate, http_header_registry);

                    tao_app_handle.manage(delegate.clone());
                }

                let ctx_clone = ctx.clone();
                let app = {
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

                    let app = AppBuilder::<TauriAppRuntime<R>>::new(
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
                    .build(&app_init_ctx, &delegate)
                    .await;

                    app
                };

                tao_app_handle.manage({
                    let ctx = ctx.unfreeze().expect("Failed to unfreeze the root context");
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
            commands::list_languages,
            commands::list_extensions,
            commands::get_translation_namespace,
            commands::welcome__open_workspace,
            commands::update_workspace,
            commands::create_workspace,
            commands::list_workspaces,
            commands::delete_workspace,
            commands::close_workspace,
            commands::update_profile,
            //
            // Main
            //
            commands::main__cancel_request,
            //
            // Welcome
            //
            commands::welcome__cancel_request,
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
            commands::create_project_resource,
            commands::delete_project_resource,
            commands::stream_project_resources,
            commands::update_project_resource,
            commands::describe_project_resource,
            commands::batch_update_project_resource,
            commands::batch_create_project_resource,
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
            WindowEvent::CloseRequested { .. } => {}

            _ => (),
        })
        .build(tauri::generate_context!())
        .expect("failed to run")
        .run(|app_handle, event| match event {
            RunEvent::Ready => {
                futures::executor::block_on(async {
                    let app = app_handle.state::<Arc<sapic_app::App<TauriAppRuntime<R>>>>();
                    let app_delegate = app_handle
                        .state::<AppDelegate<TauriAppRuntime<R>>>()
                        .inner()
                        .clone();

                    app.ensure_welcome(&app_delegate).await.unwrap();
                });
            }

            RunEvent::Exit => {}

            _ => {}
        });
}
