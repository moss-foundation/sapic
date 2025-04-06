use moss_fs::RealFileSystem;
use moss_testutils::random_name::{random_string, random_workspace_name};
use moss_workbench::activity_indicator::ActivityIndicator;
use moss_workspace::workspace::Workspace;
use moss_workspace::workspace_manager::WorkspaceManager;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::test::MockRuntime;

pub fn random_workspaces_path() -> PathBuf {
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

pub async fn setup_test_workspace_manager() -> (PathBuf, WorkspaceManager<MockRuntime>) {
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    let fs = Arc::new(RealFileSystem::new());
    let workspaces_path: PathBuf = random_workspaces_path();
    tokio::fs::create_dir_all(workspaces_path.clone())
        .await
        .unwrap();

    let workspace_manager = WorkspaceManager::new(app_handle, fs, workspaces_path.clone()).unwrap();

    (workspaces_path, workspace_manager)
}

pub async fn setup_test_workspace() -> (PathBuf, Workspace<MockRuntime>) {
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    let fs = Arc::new(RealFileSystem::new());
    let workspace_path: PathBuf = random_workspace_path();
    fs::create_dir_all(&workspace_path).unwrap();
    let activity_indicator = ActivityIndicator::new(app_handle.clone());
    let workspace =
        Workspace::new(workspace_path.clone(), fs, app_handle, activity_indicator).unwrap();
    (workspace_path, workspace)
}
