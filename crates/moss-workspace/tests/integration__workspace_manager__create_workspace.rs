mod shared;

use moss_fs::adapters::disk::DiskFileSystem;
use shared::random_workspace_name;
use std::path::PathBuf;
use std::sync::Arc;

use moss_workspace::models::operations::CreateWorkspaceInput;
use moss_workspace::workspace_manager::*;

#[tokio::test]
async fn create_workspace_success() {
    let fs = Arc::new(DiskFileSystem::new());
    let workspaces_path: PathBuf =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data");

    let workspace_manager = WorkspaceManager::new(fs, workspaces_path.clone());

    let workspace_name = random_workspace_name();
    let result = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await;

    assert!(result.is_ok());

    // Clean up
    {
        std::fs::remove_dir_all(workspaces_path.join(workspace_name)).unwrap();
    }
}

#[tokio::test]
async fn create_workspace_empty_name() {
    let fs = Arc::new(DiskFileSystem::new());
    let workspaces_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data");

    let workspace_manager = WorkspaceManager::new(fs, workspaces_path.clone());

    let result = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: "".to_string(),
        })
        .await;

    assert!(matches!(result, Err(OperationError::Validation(_))));
}

#[tokio::test]
async fn create_workspace_duplicate_name() {
    let fs = Arc::new(DiskFileSystem::new());
    let workspaces_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data");

    let workspace_manager = WorkspaceManager::new(fs, workspaces_path.clone());
    let workspace_name = random_workspace_name();

    // Create first workspace
    let result = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await;
    assert!(result.is_ok());

    // Try to create workspace with same name
    let result = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await;

    match result {
        Err(OperationError::AlreadyExists { key, path }) => {
            assert_eq!(key, PathBuf::from(&workspace_name));
            assert_eq!(path, workspaces_path.join(&workspace_name));
        }
        _ => panic!("Expected AlreadyExists error"),
    }

    // Clean up
    {
        std::fs::remove_dir_all(workspaces_path.join(workspace_name)).unwrap();
    }
}

#[tokio::test]
async fn create_workspace_invalid_path() {
    let fs = Arc::new(DiskFileSystem::new());
    let workspaces_path: PathBuf = PathBuf::from("/nonexistent/path");

    let workspace_manager = WorkspaceManager::new(fs, workspaces_path.clone());

    let result = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: random_workspace_name(),
        })
        .await;

    assert!(matches!(result, Err(OperationError::Unknown(_))));
}

#[tokio::test]
async fn create_workspace_special_chars() {
    let fs = Arc::new(DiskFileSystem::new());
    let workspaces_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data");

    let workspace_manager = WorkspaceManager::new(fs, workspaces_path.clone());

    // Test with various special characters
    let invalid_names = vec![
        "workspace/name",  // Contains path separator
        "workspace\\name", // Contains backslash
        "workspace:name",  // Contains colon
        "workspace*name",  // Contains wildcard
        "workspace?name",  // Contains question mark
        "workspace\"name", // Contains quotes
        "workspace<name",  // Contains angle brackets
        "workspace>name",  // Contains angle brackets
        "workspace|name",  // Contains pipe
    ];

    // TODO: Implement this
}
