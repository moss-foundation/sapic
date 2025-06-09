use moss_fs::RealFileSystem;
use moss_logging::LoggingService;
use moss_testutils::random_name::random_string;
use std::{fs::create_dir_all, path::PathBuf, sync::Arc};
use tauri::Manager;
use uuid::Uuid;

fn random_app_log_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}

pub async fn set_up_logging_service() -> (LoggingService, PathBuf) {
    let test_app_log_path = random_app_log_path();
    create_dir_all(test_app_log_path.clone()).unwrap();

    let fs = Arc::new(RealFileSystem::new());
    let mock_app = tauri::test::mock_app();
    let session_id = Uuid::new_v4();
    let logging_service = LoggingService::new(
        fs,
        mock_app.app_handle().clone(),
        &test_app_log_path,
        &session_id,
    )
    .unwrap();

    (logging_service, test_app_log_path)
}
