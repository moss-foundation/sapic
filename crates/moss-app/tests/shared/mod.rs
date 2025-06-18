use moss_app::{
    app::{App, AppBuilder, AppDefaults},
    models::{
        primitives::ThemeMode,
        types::{ColorThemeInfo, LocaleInfo},
    },
    services::log_service::LogService,
};
use moss_fs::RealFileSystem;
use moss_storage::global_storage::GlobalStorageImpl;
use moss_testutils::random_name::random_string;
use moss_workbench::workbench::{Options as WorkbenchOptions, Workbench};
use std::{fs::create_dir_all, path::PathBuf, sync::Arc};
use tauri::test::MockRuntime;
use uuid::Uuid;

fn random_test_app_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}

pub async fn set_up_test_app() -> (App<MockRuntime>, PathBuf) {
    let fs = Arc::new(RealFileSystem::new());
    let tauri_app = tauri::test::mock_app();
    let app_handle = tauri_app.handle().to_owned();

    let app_path = random_test_app_path();
    let applog_path = app_path.join("logs");
    create_dir_all(&applog_path).unwrap();

    let global_storage = Arc::new(GlobalStorageImpl::new(app_path.clone()).unwrap());
    let workbench = Workbench::new(
        app_handle.clone(),
        global_storage.clone(),
        WorkbenchOptions {
            abs_path: app_path.clone().into(),
        },
    );

    let session_id = Uuid::new_v4();
    let log_service = LogService::new(
        fs.clone(),
        app_handle.clone(),
        &applog_path,
        &session_id,
        global_storage.clone(),
    )
    .unwrap();

    let app_builder = AppBuilder::new(
        app_handle.clone(),
        workbench,
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
    )
    .with_service(log_service);

    (app_builder.build(), app_path)
}
