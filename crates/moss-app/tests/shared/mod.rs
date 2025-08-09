use moss_app::{App, AppBuilder, builder::BuildAppParams};
use moss_applib::{
    context::{AsyncContext, MutableContext},
    mock::MockAppRuntime,
};
use moss_fs::{FileSystem, RealFileSystem};
use moss_testutils::random_name::random_string;
use std::{future::Future, path::PathBuf, pin::Pin, sync::Arc, time::Duration};

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

pub fn random_app_dir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}

pub async fn set_up_test_app() -> (App<MockAppRuntime>, AsyncContext, CleanupFn) {
    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();

    let fs = Arc::new(RealFileSystem::new());
    let tauri_app = tauri::test::mock_app();
    let app_handle = tauri_app.handle().to_owned();

    <dyn FileSystem>::set_global(fs.clone(), &app_handle);

    let app_path = random_app_dir_path();

    let logs_abs_path = app_path.join("logs");
    let workspaces_abs_path = app_path.join("workspaces");
    let globals_abs_path = app_path.join("globals");
    let themes_abs_path = app_path.join("themes");
    let locales_abs_path = app_path.join("locales");

    {
        tokio::fs::create_dir_all(&app_path).await.unwrap();
        tokio::fs::create_dir(&logs_abs_path).await.unwrap();
        tokio::fs::create_dir(&workspaces_abs_path).await.unwrap();
        tokio::fs::create_dir(&globals_abs_path).await.unwrap();
        tokio::fs::create_dir(&themes_abs_path).await.unwrap();
        tokio::fs::create_dir(&locales_abs_path).await.unwrap();

        tokio::fs::write(&themes_abs_path.join("themes.json"), THEMES)
            .await
            .unwrap();
        tokio::fs::write(&locales_abs_path.join("locales.json"), LOCALES)
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
        AppBuilder::<MockAppRuntime>::new(app_handle.clone(), fs.clone())
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
        ctx,
        cleanup_fn,
    )
}
