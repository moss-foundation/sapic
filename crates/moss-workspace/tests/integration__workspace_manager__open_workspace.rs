mod shared;

use moss_testutils::random_name::random_workspace_name;
use moss_workspace::models::operations::{CreateWorkspaceInput, OpenWorkspaceInput};
use moss_workspace::workspace_manager::OperationError;
use std::path::PathBuf;

use crate::shared::setup_test_workspace_manager;

#[tokio::test]
async fn open_workspace_success() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let first_workspace_name = random_workspace_name();
    let first_workspace_path = workspaces_path.join(&first_workspace_name);

    workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: first_workspace_name.clone(),
        })
        .await
        .unwrap();

    let second_workspace_name = random_workspace_name();
    workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: second_workspace_name.clone(),
        })
        .await
        .unwrap();

    // Opening the first workspace
    let result = workspace_manager
        .open_workspace(OpenWorkspaceInput {
            path: first_workspace_path.clone(),
        })
        .await;
    assert!(result.is_ok());

    let current_workspace = workspace_manager.current_workspace().unwrap();
    assert_eq!(current_workspace.1.path(), first_workspace_path);

    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }
}

#[tokio::test]
async fn open_workspace_not_found() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let result = workspace_manager
        .open_workspace(OpenWorkspaceInput {
            path: PathBuf::from("nonexistent/path"),
        })
        .await;
    assert!(matches!(result, Err(OperationError::NotFound { .. })));

    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }
}

#[tokio::test]
async fn open_workspace_already_active() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);
    workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await
        .unwrap();

    let result = workspace_manager
        .open_workspace(OpenWorkspaceInput {
            path: expected_path.clone(),
        })
        .await;

    assert!(result.is_ok());

    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }
}
