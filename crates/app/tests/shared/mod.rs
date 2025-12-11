#![cfg(feature = "integration-tests")]

use app::{App, builder::AppBuilder};
use moss_app_delegate::AppDelegate;
use moss_applib::mock::MockAppRuntime;
use moss_fs::RealFileSystem;
use moss_keyring::KeyringClientImpl;
use moss_testutils::random_name::random_string;
use reqwest::ClientBuilder as HttpClientBuilder;
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
    language::language_registry::AppLanguagePackRegistry,
    ports::{github_api::GitHubAuthAdapter, gitlab_api::GitLabAuthAdapter},
    theme::theme_registry::AppThemeRegistry,
    user::User,
};
use std::{path::PathBuf, pin::Pin, sync::Arc, time::Duration};
use tauri::Manager;

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub async fn setup_test_app() -> (
    App<MockAppRuntime>,
    AppDelegate<MockAppRuntime>,
    ArcContext,
    CleanupFn,
) {
    let test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10));
    let resources_path = test_path.join("resources");
    let builtin_extensions_path = resources_path.join("extensions");
    tokio::fs::create_dir_all(test_path.clone()).await.unwrap();
    tokio::fs::create_dir_all(resources_path.clone())
        .await
        .unwrap();
    tokio::fs::create_dir_all(builtin_extensions_path.clone())
        .await
        .unwrap();

    let keyring = Arc::new(KeyringClientImpl::new());
    let http_client = HttpClientBuilder::new()
        .user_agent("SAPIC/1.0")
        .build()
        .expect("failed to build http client");

    let server_api_endpoint = "Placeholder";

    let tauri_app = tauri::test::mock_app();
    let tao_app_handle = tauri_app.handle().to_owned();

    let delegate = AppDelegate::<MockAppRuntime>::new(tao_app_handle.clone());
    delegate.set_user_dir(test_path.clone());
    delegate.set_resource_dir(resources_path.clone());

    let fs = Arc::new(RealFileSystem::new(&delegate.tmp_dir()));

    let kv_storage = AppStorage::new(&delegate.globals_dir(), delegate.workspaces_dir(), None)
        .await
        .expect("failed to create storage");

    let theme_registry = AppThemeRegistry::new();
    let language_registry = AppLanguagePackRegistry::new();
    let configuration_registry =
        AppConfigurationRegistry::new().expect("failed to build configuration registry");

    let server_api_client: Arc<HttpServerApiClient> =
        HttpServerApiClient::new(server_api_endpoint.to_string(), http_client.clone()).into();

    let github_api_client = Arc::new(AppGitHubApiClient::new(http_client.clone()));
    let gitlab_api_client = Arc::new(AppGitLabApiClient::new(http_client.clone()));

    let auth_gateway_url: Arc<String> = server_api_client.base_url().to_string().into();

    let github_auth_adapter: Arc<dyn GitHubAuthAdapter> = Arc::new(AppGitHubAuthAdapter::new(
        server_api_client.clone(),
        auth_gateway_url.clone(),
        8080,
    ));
    let gitlab_auth_adapter: Arc<dyn GitLabAuthAdapter> = Arc::new(AppGitLabAuthAdapter::new(
        server_api_client.clone(),
        auth_gateway_url,
        8081,
    ));

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
    .await
    .unwrap();

    let settings_storage = AppSettingsStorage::new(configuration_registry.clone(), user.settings());

    GlobalLanguagePackRegistry::set(&delegate, language_registry.clone());
    GlobalThemeRegistry::set(&delegate, theme_registry.clone());
    GlobalConfigurationRegistry::set(&delegate, configuration_registry);
    GlobalSettingsStorage::set(&delegate, Arc::new(settings_storage));
    GlobalKvStorage::set(&delegate, kv_storage.clone());

    tao_app_handle.manage(delegate.clone());

    let ctx = ArcContext::background();
    let ctx_clone = ctx.clone();
    let app = {
        let app_init_ctx = ArcContext::new_with_timeout(ctx_clone, Duration::from_secs(30));
        let app = AppBuilder::<MockAppRuntime>::new(
            user,
            fs,
            keyring,
            vec![],
            server_api_client,
            github_api_client,
            gitlab_api_client,
            kv_storage.clone(),
            theme_registry,
            language_registry,
        )
        .build(&app_init_ctx, &delegate)
        .await;

        app
    };

    let cleanup_fn = Box::new({
        let path = test_path.clone();
        let kv_storage_clone = kv_storage.clone();
        move || {
            Box::pin(async move {
                kv_storage_clone.close().await.unwrap();
                if let Err(e) = tokio::fs::remove_dir_all(path).await {
                    eprintln!("Failed to clean up test directory: {}", e);
                }
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        }
    });

    (app, delegate, ctx, cleanup_fn)
}
