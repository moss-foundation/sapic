mod shared;

use moss_testutils::random_name::random_workspace_name;
use moss_workspace::models::operations::{CreateWorkspaceInput, DeleteWorkspaceInput};

use crate::shared::setup_test_workspace_manager;

#[tokio::test]
async fn delete_workspace_success() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);
    let create_workspace_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await
        .unwrap();

    let key = create_workspace_output.key;
    let delete_workspace_result = workspace_manager
        .delete_workspace(&DeleteWorkspaceInput { key })
        .await;
    assert!(delete_workspace_result.is_ok());

    // Check folder is removed
    assert!(!expected_path.exists());

    // Check removing current workspace
    let current_workspace = workspace_manager.current_workspace();
    assert!(current_workspace.is_err());

    // Check updating known_workspaces
    let list_workspaces_output = workspace_manager.list_workspaces().await.unwrap();
    assert!(list_workspaces_output.0.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn delete_workspace_nonexistent_key() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let create_workspace_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await
        .unwrap();
    let key = create_workspace_output.key;

    workspace_manager
        .delete_workspace(&DeleteWorkspaceInput { key })
        .await
        .unwrap();

    let delete_workspace_result = workspace_manager
        .delete_workspace(&DeleteWorkspaceInput { key })
        .await;
    assert!(delete_workspace_result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn delete_workspace_fs_already_deleted() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);
    let create_workspace_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await
        .unwrap();
    let key = create_workspace_output.key;

    // Delete the workspace folder
    tokio::fs::remove_dir_all(expected_path).await.unwrap();

    // This should simply be a no-op
    let delete_workspace_result = workspace_manager
        .delete_workspace(&DeleteWorkspaceInput { key })
        .await;
    assert!(delete_workspace_result.is_ok());

    // Check removing current workspace
    let current_workspace = workspace_manager.current_workspace();
    assert!(current_workspace.is_err());

    // Check updating known_workspaces
    let workspaces_list = workspace_manager.list_workspaces().await.unwrap();
    assert!(workspaces_list.0.is_empty());

    cleanup().await;
}
