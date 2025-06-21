mod context;

pub use context::*;

use moss_activity_indicator::ActivityIndicator;
use moss_app::{
    app::{App, AppBuilder, AppDefaults},
    models::{
        primitives::ThemeMode,
        types::{ColorThemeInfo, LocaleInfo},
    },
    services::{log_service::LogService, workspace_service::WorkspaceService},
    storage::segments::WORKSPACE_SEGKEY,
};
use moss_fs::{FileSystem, RealFileSystem};
use moss_storage::{global_storage::GlobalStorageImpl, primitives::segkey::SegKeyBuf};
use moss_testutils::random_name::random_string;
use std::{future::Future, path::PathBuf, pin::Pin, sync::Arc};
use tauri::test::MockRuntime;
use uuid::Uuid;

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub fn random_app_dir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}

pub fn workspace_key(id: Uuid) -> SegKeyBuf {
    WORKSPACE_SEGKEY.join(id.to_string())
}

pub async fn set_up_test_app() -> (App<MockRuntime>, MockAppContext, CleanupFn, PathBuf) {
    let fs = Arc::new(RealFileSystem::new());
    let tauri_app = tauri::test::mock_app();
    let app_handle = tauri_app.handle().to_owned();

    <dyn FileSystem>::set_global(fs.clone(), &app_handle);

    let app_path = random_app_dir_path();

    let logs_abs_path = app_path.join("logs");
    let workspaces_abs_path = app_path.join("workspaces");
    let globals_abs_path = app_path.join("globals");

    {
        tokio::fs::create_dir_all(&app_path).await.unwrap();
        tokio::fs::create_dir(&logs_abs_path).await.unwrap();
        tokio::fs::create_dir(&workspaces_abs_path).await.unwrap();
        tokio::fs::create_dir(&globals_abs_path).await.unwrap();
    }

    let global_storage = Arc::new(GlobalStorageImpl::new(app_path.clone()).unwrap());

    let session_id = Uuid::new_v4();
    let log_service = LogService::new(
        fs.clone(),
        app_handle.clone(),
        &logs_abs_path,
        &session_id,
        global_storage.clone(),
    )
    .unwrap();

    let workspace_service: WorkspaceService<MockRuntime> =
        WorkspaceService::new(global_storage.clone(), fs.clone(), &app_path);

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

    // FIXME: This is a hack, should be a mock
    let activity_indicator = ActivityIndicator::new(app_handle.clone());
    let ctx = MockAppContext::new(app_handle.clone());
    let app_builder = AppBuilder::new(
        app_handle.clone(),
        global_storage,
        activity_indicator,
        AppDefaults {
            theme: ColorThemeInfo {
                identifier: "".to_string(),
                display_name: "".to_string(),
                mode: ThemeMode::Light,
                order: None,
                source: Default::default(),
                is_default: None,
            },
            locale: LocaleInfo {
                identifier: "".to_string(),
                display_name: "".to_string(),
                code: "".to_string(),
                direction: None,
                is_default: None,
            },
        },
        fs.clone(),
        app_path.clone(),
    )
    .with_service(log_service)
    .with_service(workspace_service);

    (
        app_builder.build().await.unwrap(),
        ctx,
        cleanup_fn,
        app_path,
    )
}
