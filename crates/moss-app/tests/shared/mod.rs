use moss_app::services::log_service::LogService;
use moss_fs::RealFileSystem;
use moss_storage::global_storage::GlobalStorageImpl;
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

pub async fn set_up_log_service() -> (LogService, PathBuf) {
    let test_app_log_path = random_app_log_path();
    create_dir_all(test_app_log_path.clone()).unwrap();

    let fs = Arc::new(RealFileSystem::new());
    let mock_app = tauri::test::mock_app();
    let session_id = Uuid::new_v4();
    let storage = Arc::new(GlobalStorageImpl::new(&test_app_log_path).unwrap());
    let log_service = LogService::new(
        fs,
        mock_app.app_handle().clone(),
        &test_app_log_path,
        &session_id,
        storage.clone(),
    )
    .unwrap();

    (log_service, test_app_log_path)
}
