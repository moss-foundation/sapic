pub mod shared;

use crate::shared::setup_test_workspace_manager;
use moss_common::api::OperationError;
use moss_testutils::random_name::random_workspace_name;
use moss_workbench::models::operations::{
    CloseWorkspaceInput, CreateWorkspaceInput, OpenWorkspaceInput,
};
use moss_workspace::models::types::WorkspaceMode;
use uuid::Uuid;

#[tokio::test]
async fn test_diagnostic_create_workspace_only() {
    let (ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();

    // Only create workspace WITHOUT opening it
    let create_result = workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await;

    assert!(create_result.is_ok());
    let _create_output = create_result.unwrap();

    cleanup().await;
}

#[tokio::test]
async fn test_diagnostic_create_and_open_workspace() {
    let (ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();

    // Create workspace WITHOUT opening it
    let create_result = workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await;

    assert!(create_result.is_ok());
    let create_output = create_result.unwrap();

    // Now try to open it
    let open_result = workspace_manager
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;

    assert!(open_result.is_ok());

    cleanup().await;
}

#[tokio::test]
async fn close_workspace_success() {
    let (ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();

    // Create workspace WITHOUT opening it first
    let create_result = workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await;
    assert!(create_result.is_ok());
    let create_output = create_result.unwrap();

    // Manually open the workspace
    let open_result = workspace_manager
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;
    assert!(open_result.is_ok());

    // Verify workspace is active (in a block to release the read lock)
    {
        let active_workspace = workspace_manager.active_workspace().await;
        assert!(active_workspace.as_ref().is_some());
        let active_workspace = active_workspace.as_ref().unwrap();
        assert_eq!(active_workspace.id, create_output.id);
    }

    // Close the workspace
    let close_result = workspace_manager
        .close_workspace(&CloseWorkspaceInput {
            id: create_output.id,
        })
        .await;

    assert!(close_result.is_ok());

    let close_output = close_result.unwrap();
    assert_eq!(close_output.id, create_output.id);

    // Verify no workspace is active
    let active_workspace = workspace_manager.active_workspace().await;
    assert!(active_workspace.as_ref().is_none());

    cleanup().await;
}

#[tokio::test]
async fn close_workspace_no_active_workspace() {
    let (_ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    // Try to close a workspace when none is active
    let close_result = workspace_manager
        .close_workspace(&CloseWorkspaceInput { id: Uuid::new_v4() })
        .await;

    assert!(close_result.is_err());
    assert!(matches!(close_result, Err(OperationError::InvalidInput(_))));

    let error_message = match close_result {
        Err(OperationError::InvalidInput(msg)) => msg,
        _ => panic!("Expected InvalidInput error"),
    };
    assert!(error_message.contains("No active workspace to close"));

    cleanup().await;
}

#[tokio::test]
async fn close_workspace_wrong_id() {
    let (ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();

    // Create and open a workspace
    let create_result = workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await;
    assert!(create_result.is_ok());
    let create_output = create_result.unwrap();

    // Open the workspace
    let open_result = workspace_manager
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;
    assert!(open_result.is_ok());

    // Verify workspace is active
    let active_workspace = workspace_manager.active_workspace().await;
    assert!(active_workspace.as_ref().is_some());
    assert_eq!(active_workspace.as_ref().unwrap().id, create_output.id);

    // Try to close with wrong ID (this should work fast without hanging)
    let wrong_id = Uuid::new_v4();
    let close_result = workspace_manager
        .close_workspace(&CloseWorkspaceInput { id: wrong_id })
        .await;

    assert!(close_result.is_err());
    assert!(matches!(close_result, Err(OperationError::InvalidInput(_))));

    let error_message = match close_result {
        Err(OperationError::InvalidInput(msg)) => msg,
        _ => panic!("Expected InvalidInput error"),
    };
    assert!(error_message.contains("is not currently active"));

    // Verify the original workspace is still active
    let active_workspace = workspace_manager.active_workspace().await;
    assert!(active_workspace.as_ref().is_some());
    assert_eq!(active_workspace.as_ref().unwrap().id, create_output.id);

    cleanup().await;
}
