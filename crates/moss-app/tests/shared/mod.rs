use moss_app::{App, AppBuilder, builder::BuildAppParams};
use moss_app_delegate::AppDelegate;
use moss_applib::{
    context::{AsyncContext, MutableContext},
    mock::MockAppRuntime,
};
use moss_fs::RealFileSystem;
use moss_keyring::test::MockKeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_testutils::random_name::random_string;
use reqwest::ClientBuilder as HttpClientBuilder;
use std::{future::Future, path::PathBuf, pin::Pin, sync::Arc, time::Duration};
use tauri::Manager;

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

const THEMES: &str = r#"
[
    {
        "identifier": "moss.sapic-theme.lightDefault",
        "displayName": "Light Default",
        "mode": "light",
        "order": 1,
        "source": "light.css",
        "isDefault": true
    }
]
"#;

const LOCALES: &str = r#"
[
    {
    "identifier": "moss.sapic-locale.en",
    "displayName": "English",
    "code": "en",
    "direction": "ltr",
    "isDefault": true
    }
]
"#;

const PROFILES: &str = r#"
{
  "name": "Default",
  "accounts": []
}
"#;

const ACCOUNT_AUTH_BASE_URL: &str = "https://account-auth-gateway-dev.20g10z3r.workers.dev";

pub fn random_app_dir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}

pub async fn set_up_test_app() -> (
    App<MockAppRuntime>,
    AppDelegate<MockAppRuntime>,
    AsyncContext,
    CleanupFn,
) {
    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();

    let keyring = Arc::new(MockKeyringClient::new());
    let fs = Arc::new(RealFileSystem::new());
    let tauri_app = tauri::test::mock_app();
    let tao_app_handle = tauri_app.handle().to_owned();
    let http_client = HttpClientBuilder::new()
        .user_agent("SAPIC-TEST/1.0")
        .build()
        .expect("failed to build http client");
    let auth_api_client = Arc::new(AccountAuthGatewayApiClient::new(
        http_client.clone(),
        ACCOUNT_AUTH_BASE_URL.to_string(),
    ));
    let app_delegate = AppDelegate::<MockAppRuntime>::new(tao_app_handle.clone());

    {
        tao_app_handle.manage(app_delegate.clone());
    }

    let app_path = random_app_dir_path();

    let logs_abs_path = app_path.join("logs");
    let workspaces_abs_path = app_path.join("workspaces");
    let globals_abs_path = app_path.join("globals");
    let themes_abs_path = app_path.join("themes");
    let locales_abs_path = app_path.join("locales");
    let profiles_abs_path = app_path.join("profiles");

    {
        tokio::fs::create_dir_all(&app_path).await.unwrap();
        tokio::fs::create_dir(&logs_abs_path).await.unwrap();
        tokio::fs::create_dir(&workspaces_abs_path).await.unwrap();
        tokio::fs::create_dir(&globals_abs_path).await.unwrap();
        tokio::fs::create_dir(&themes_abs_path).await.unwrap();
        tokio::fs::create_dir(&locales_abs_path).await.unwrap();
        tokio::fs::create_dir(&profiles_abs_path).await.unwrap();

        tokio::fs::write(&themes_abs_path.join("themes.json"), THEMES)
            .await
            .unwrap();
        tokio::fs::write(&locales_abs_path.join("locales.json"), LOCALES)
            .await
            .unwrap();
        tokio::fs::write(&profiles_abs_path.join("e_MChWGYcY.json"), PROFILES)
            .await
            .unwrap();
    }

    let cleanup_fn = Box::new({
        let path = app_path.clone();
        move || {
            Box::pin(async move {
                if let Err(e) = tokio::fs::remove_dir_all(&path).await {
                    eprintln!("Failed to clean up test directory: {}", e);
                }
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        }
    });

    (
        AppBuilder::<MockAppRuntime>::new(
            tao_app_handle.clone(),
            fs.clone(),
            keyring,
            auth_api_client,
        )
        .build(
            &ctx,
            BuildAppParams {
                app_dir: app_path.clone(),
                themes_dir: themes_abs_path,
                locales_dir: locales_abs_path,
                logs_dir: logs_abs_path,
            },
        )
        .await,
        app_delegate,
        ctx,
        cleanup_fn,
    )
}
