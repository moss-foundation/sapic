mod shared;

use moss_common::api::OperationError;
use moss_testutils::random_name::random_workspace_name;
use moss_workbench::models::operations::{CreateWorkspaceInput, OpenWorkspaceInput};
use moss_workspace::models::types::WorkspaceMode;
use std::path::Path;
use std::sync::Arc;

use crate::shared::setup_test_workspace_manager;

#[tokio::test]
async fn open_workspace_success() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let first_workspace_name = random_workspace_name();
    let first_workspace_path: Arc<Path> = workspaces_path.join(&first_workspace_name).into();

    workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: first_workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await
        .unwrap();

    let second_workspace_name = random_workspace_name();
    workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: second_workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await
        .unwrap();

    // Opening the first workspace
    let open_workspace_result = workspace_manager
        .open_workspace(&OpenWorkspaceInput {
            name: first_workspace_name.clone(),
        })
        .await;
    assert!(open_workspace_result.is_ok());

    let open_workspace_output = open_workspace_result.unwrap();
    assert_eq!(open_workspace_output.abs_path, first_workspace_path);

    // Check that active workspace has the correct ID
    let active_workspace = workspace_manager.active_workspace().unwrap();
    assert_eq!(active_workspace.id, open_workspace_output.id);

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_not_found() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let open_workspace_result = workspace_manager
        .open_workspace(&OpenWorkspaceInput {
            name: "nonexistent".to_string(),
        })
        .await;
    assert!(open_workspace_result.is_err());
    assert!(matches!(
        open_workspace_result,
        Err(OperationError::NotFound { .. })
    ));

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_already_active() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path: Arc<Path> = workspaces_path.join(&workspace_name).into();

    // Create workspace with open_on_creation=true to make it active immediately
    let create_workspace_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await
        .unwrap();

    // Get the workspace ID
    let workspace_id = create_workspace_output.id;

    // Try to open the same workspace again
    let open_workspace_result = workspace_manager
        .open_workspace(&OpenWorkspaceInput {
            name: workspace_name,
        })
        .await;

    assert!(open_workspace_result.is_ok());
    let open_workspace_output = open_workspace_result.unwrap();

    // Check if we get the same workspace ID back
    assert_eq!(open_workspace_output.id, workspace_id);
    assert_eq!(open_workspace_output.abs_path, expected_path);

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_fs_already_deleted() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let workspace_path = workspaces_path.join(&workspace_name);

    // Create workspace but don't open it
    workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await
        .unwrap();

    // Delete the workspace folder
    tokio::fs::remove_dir_all(&workspace_path).await.unwrap();

    // Try to open it - should fail with NotFound
    let open_workspace_result = workspace_manager
        .open_workspace(&OpenWorkspaceInput {
            name: workspace_name,
        })
        .await;

    assert!(open_workspace_result.is_err());
    assert!(matches!(
        open_workspace_result,
        Err(OperationError::NotFound { .. })
    ));

    cleanup().await;
}
