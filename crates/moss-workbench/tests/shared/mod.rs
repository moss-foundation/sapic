use moss_applib::context::test::MockContext;
use moss_fs::{FileSystem, RealFileSystem};
use moss_storage::{global_storage::GlobalStorageImpl, primitives::segkey::SegKeyBuf};
use moss_testutils::random_name::random_string;
use moss_workbench::{
    storage::segments::WORKSPACE_SEGKEY,
    workbench::{self, Workbench},
};
use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tauri::test::MockRuntime;
use uuid::Uuid;

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub fn random_app_dir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}

pub async fn setup_test_workspace_manager()
-> (MockContext, Arc<Path>, Workbench<MockRuntime>, CleanupFn) {
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    let fs = Arc::new(RealFileSystem::new());

    <dyn FileSystem>::set_global(fs, &app_handle);

    let ctx = MockContext::new(app_handle.clone());

    let random_abs_app_path: Arc<Path> = random_app_dir_path().into();
    let workspaces_abs_path: Arc<Path> = random_abs_app_path.join("workspaces").into();
    let globals_abs_path = random_abs_app_path.join("globals");

    {
        tokio::fs::create_dir_all(&random_abs_app_path)
            .await
            .unwrap();
        tokio::fs::create_dir(&workspaces_abs_path).await.unwrap();
        tokio::fs::create_dir(&globals_abs_path).await.unwrap();
    }

    let global_storage = Arc::new(GlobalStorageImpl::new(&globals_abs_path).unwrap());

    let workspace_manager = Workbench::new(
        app_handle,
        global_storage,
        workbench::Options {
            abs_path: random_abs_app_path.clone(),
        },
    );

    let app_dir = random_abs_app_path.clone();
    let cleanup_fn = Box::new(move || {
        let path = app_dir;
        Box::pin(async move {
            if let Err(e) = tokio::fs::remove_dir_all(&path).await {
                eprintln!("Failed to clean up test directory: {}", e);
            }
        }) as Pin<Box<dyn Future<Output = ()> + Send>>
    });

    (ctx, workspaces_abs_path, workspace_manager, cleanup_fn)
}

pub fn workspace_key(id: Uuid) -> SegKeyBuf {
    WORKSPACE_SEGKEY.join(id.to_string())
}
