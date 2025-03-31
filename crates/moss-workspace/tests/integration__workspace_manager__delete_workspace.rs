mod shared;

use moss_testutils::random_name::random_workspace_name;
use moss_workspace::models::operations::{CreateWorkspaceInput, DeleteWorkspaceInput};

use crate::shared::setup_test_workspace_manager;

#[tokio::test]
async fn delete_workspace_success() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);
    let output = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await
        .unwrap();
    let key = output.key;
    let result = workspace_manager
        .delete_workspace(DeleteWorkspaceInput { key })
        .await;
    assert!(result.is_ok());

    // Check folder is removed
    assert!(!expected_path.exists());

    // Check removing current workspace
    let current_workspace = workspace_manager.current_workspace();
    assert!(current_workspace.is_err());

    // Check updating known_workspaces
    let workspaces_list = workspace_manager.list_workspaces().await.unwrap();
    assert!(workspaces_list.0.is_empty());

    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }
}

#[tokio::test]
async fn delete_workspace_nonexistent_key() {
    // FIXME: Should this be an error or a no-op, since technically what needs to be deleted is gone?
    // This might happen, e.g., when the frontend tries to delete already deleted workspace
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);
    let output = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await
        .unwrap();
    let key = output.key;

    workspace_manager
        .delete_workspace(DeleteWorkspaceInput { key })
        .await
        .unwrap();

    let result = workspace_manager
        .delete_workspace(DeleteWorkspaceInput { key })
        .await;
    assert!(result.is_err());

    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }
}

#[tokio::test]
async fn delete_workspace_fs_already_deleted() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);
    let key = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await
        .unwrap()
        .key;

    // Delete the workspace folder
    std::fs::remove_dir_all(expected_path).unwrap();

    // This should simply be a no-op
    let result = workspace_manager
        .delete_workspace(DeleteWorkspaceInput { key })
        .await;
    assert!(result.is_ok());

    // Check removing current workspace
    let current_workspace = workspace_manager.current_workspace();
    assert!(current_workspace.is_err());

    // Check updating known_workspaces
    let workspaces_list = workspace_manager.list_workspaces().await.unwrap();
    assert!(workspaces_list.0.is_empty());

    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }
}
