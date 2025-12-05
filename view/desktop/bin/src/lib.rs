mod commands;
mod constants;
mod mem;
mod menu;
mod plugins;

#[macro_use]
extern crate tracing;

use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::TauriAppRuntime;
use moss_extension_points::{
    configurations::ConfigurationExtensionPoint, http_headers::HttpHeadersExtensionPoint,
    languages::LanguageExtensionPoint, resource_statuses::ResourceStatusesExtensionPoint,
    themes::ThemeExtensionPoint,
};
use moss_fs::RealFileSystem;
use moss_keyring::KeyringClientImpl;
use moss_project::registries::{
    http_headers::{AppHttpHeaderRegistry, HttpHeaderRegistry},
    resource_statuses::{AppResourceStatusRegistry, ResourceStatusRegistry},
};
use reqwest::ClientBuilder as HttpClientBuilder;
use sapic_app::{builder::AppBuilder, command::CommandDecl};
use sapic_core::context::ArcContext;
use sapic_platform::{
    github::{AppGitHubApiClient, auth::AppGitHubAuthAdapter},
    gitlab::{AppGitLabApiClient, auth::AppGitLabAuthAdapter},
    server::HttpServerApiClient,
};
use sapic_runtime::{
    app::{kv_storage::AppStorage, settings_storage::AppSettingsStorage},
    globals::{
        GlobalConfigurationRegistry, GlobalKvStorage, GlobalLanguagePackRegistry,
        GlobalSettingsStorage, GlobalThemeRegistry,
    },
    user::AppUser,
};
use sapic_system::{
    configuration::configuration_registry::AppConfigurationRegistry,
    language::{
        language_registry::AppLanguagePackRegistry,
        language_service::RegisterTranslationContribution,
    },
    ports::{github_api::GitHubAuthAdapter, gitlab_api::GitLabAuthAdapter},
    theme::theme_registry::AppThemeRegistry,
    user::User,
};
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
        .plugin(tauri_plugin_opener::init())
        .plugin(shared_storage::init(|app| {
            let handle = app
                .downcast::<R>()
                .ok_or_join_err::<()>("failed to downcast app handle")?;

            let delegate = handle
                .state::<AppDelegate<TauriAppRuntime<R>>>()
                .inner()
                .clone();

            Ok(GlobalKvStorage::get(&delegate))
        }))
        .plugin(settings_storage::init(|app| {
            let handle = app
                .downcast::<R>()
                .ok_or_join_err::<()>("failed to downcast app handle")?;

            let delegate = handle
                .state::<AppDelegate<TauriAppRuntime<R>>>()
                .inner()
                .clone();

            Ok(GlobalSettingsStorage::get(&delegate))
        }));

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(mac_window::init());
    }

    builder
        .setup(|tao| {
            futures::executor::block_on(async {
                let ctx = ArcContext::background();

                let keyring = Arc::new(KeyringClientImpl::new());
                let http_client = HttpClientBuilder::new()
                    .user_agent("SAPIC/1.0")
                    .build()
                    .expect("failed to build http client");

                let server_api_endpoint =
                    dotenvy::var("SERVER_API_ENDPOINT").expect("SERVER_API_ENDPOINT is not set");

                let tao_app_handle = tao.app_handle();

                let delegate = AppDelegate::<TauriAppRuntime<R>>::new(tao_app_handle.clone());
                let fs = Arc::new(RealFileSystem::new(&delegate.tmp_dir()));

                let kv_storage =
                    AppStorage::new(&delegate.globals_dir(), delegate.workspaces_dir(), None)
                        .await
                        .expect("failed to create storage");

                let theme_registry = AppThemeRegistry::new();
                let language_registry = AppLanguagePackRegistry::new();
                let configuration_registry = AppConfigurationRegistry::new()
                    .expect("failed to build configuration registry");
                let resource_status_registry = AppResourceStatusRegistry::new()
                    .expect("failed to build resource status registry");
                let http_header_registry =
                    AppHttpHeaderRegistry::new().expect("failed to build http header registry");

                let server_api_client: Arc<HttpServerApiClient> =
                    HttpServerApiClient::new(server_api_endpoint, http_client.clone()).into();

                let github_api_client = Arc::new(AppGitHubApiClient::new(http_client.clone()));
                let gitlab_api_client = Arc::new(AppGitLabApiClient::new(http_client.clone()));

                let auth_gateway_url: Arc<String> = server_api_client.base_url().to_string().into();

                let github_auth_adapter: Arc<dyn GitHubAuthAdapter> =
                    Arc::new(AppGitHubAuthAdapter::new(
                        server_api_client.clone(),
                        auth_gateway_url.clone(),
                        8080,
                    ));
                let gitlab_auth_adapter: Arc<dyn GitLabAuthAdapter> = Arc::new(
                    AppGitLabAuthAdapter::new(server_api_client.clone(), auth_gateway_url, 8081),
                );

                let user = AppUser::new(
                    delegate.user_dir(),
                    fs.clone(),
                    server_api_client.clone(),
                    github_api_client.clone(),
                    gitlab_api_client.clone(),
                    github_auth_adapter.clone(),
                    gitlab_auth_adapter.clone(),
                    keyring.clone(),
                )
                .await?;

                let settings_storage =
                    AppSettingsStorage::new(configuration_registry.clone(), user.settings());

                GlobalLanguagePackRegistry::set(&delegate, language_registry.clone());
                GlobalThemeRegistry::set(&delegate, theme_registry.clone());
                GlobalConfigurationRegistry::set(&delegate, configuration_registry);
                GlobalSettingsStorage::set(&delegate, Arc::new(settings_storage));
                GlobalKvStorage::set(&delegate, kv_storage.clone());

                <dyn ResourceStatusRegistry>::set_global(&delegate, resource_status_registry);
                <dyn HttpHeaderRegistry>::set_global(&delegate, http_header_registry);

                tao_app_handle.manage(delegate.clone());

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
                        ArcContext::new_with_timeout(ctx_clone, Duration::from_secs(30));

                    let app = AppBuilder::<TauriAppRuntime<R>>::new(
                        user,
                        fs,
                        keyring,
                        vec![
                            ThemeExtensionPoint::new(),
                            LanguageExtensionPoint::new(),
                            ConfigurationExtensionPoint::new(),
                            ResourceStatusesExtensionPoint::new(),
                            HttpHeadersExtensionPoint::new(),
                        ],
                        server_api_client,
                        github_api_client,
                        gitlab_api_client,
                        kv_storage,
                        theme_registry,
                        language_registry,
                    )
                    .with_command(shortcut_println_command)
                    .with_command(shortcut_alert_command)
                    .build(&app_init_ctx, &delegate)
                    .await;

                    app
                };

                tao_app_handle.manage(ctx.clone());
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
            commands::download_extension,
            commands::get_translation_namespace,
            commands::list_workspaces,
            commands::delete_workspace,
            commands::update_profile,
            commands::list_user_accounts,
            commands::add_user_account,
            commands::update_user_account,
            commands::remove_user_account,
            //
            // Main
            //
            commands::main__cancel_request,
            commands::main__update_workspace,
            commands::main__create_workspace,
            commands::main__open_workspace,
            commands::main__close_workspace,
            //
            // Welcome
            //
            commands::welcome__cancel_request,
            commands::welcome__create_workspace,
            commands::welcome__open_workspace,
            commands::welcome__update_workspace,
            //
            // Workspace
            //
            commands::stream_environments,
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
