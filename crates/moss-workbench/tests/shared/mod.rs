use moss_activity_indicator::ActivityIndicator;
use moss_fs::RealFileSystem;
use moss_storage::global_storage::GlobalStorageImpl;
use moss_testutils::random_name::{random_string, random_workspace_name};
use moss_workbench::workbench::{self, Workbench};
use moss_workspace::workspace::Workspace;
use std::fs;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use tauri::test::MockRuntime;

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub fn random_app_dir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}

pub fn random_workspace_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("workspaces")
        .join(random_workspace_name())
}

pub async fn setup_test_workspace_manager() -> (Arc<Path>, Workbench<MockRuntime>, CleanupFn) {
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    let fs = Arc::new(RealFileSystem::new());

    let random_abs_app_path: PathBuf = random_app_dir_path();
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
        fs,
        global_storage,
        workbench::Options {
            workspaces_abs_path: workspaces_abs_path.clone(),
            next_workspace_id: Arc::new(AtomicUsize::new(0)),
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

    (workspaces_abs_path, workspace_manager, cleanup_fn)
}

pub async fn setup_test_workspace() -> (Arc<Path>, Workspace<MockRuntime>) {
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    let fs = Arc::new(RealFileSystem::new());
    let workspace_abs_path: Arc<Path> = random_workspace_path().into();
    fs::create_dir_all(&workspace_abs_path).unwrap();

    let activity_indicator = ActivityIndicator::new(app_handle.clone());
    let workspace = Workspace::new(
        app_handle.clone(),
        workspace_abs_path.clone(),
        fs,
        activity_indicator,
    )
    .unwrap();
    (workspace_abs_path, workspace)
}
