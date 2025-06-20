pub mod shared;

use moss_common::api::OperationError;
use moss_testutils::random_name::random_workspace_name;
use moss_workbench::models::operations::{CreateWorkspaceInput, UpdateWorkspaceInput};
use moss_workspace::models::types::WorkspaceMode;

use crate::shared::setup_test_workspace_manager;

#[tokio::test]
async fn rename_workspace_success() {
    let (ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let old_workspace_name = random_workspace_name();
    let create_workspace_output = workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: old_workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            },
        )
        .await
        .unwrap();
    let id = create_workspace_output.id;

    let new_workspace_name = random_workspace_name();
    let update_workspace_result = workspace_manager
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                name: Some(new_workspace_name.clone()),
            },
        )
        .await;
    assert!(update_workspace_result.is_ok());

    // Check updating active workspace
    let active_workspace = workspace_manager.active_workspace().await;
    let (workspace_guard, _context) = active_workspace.as_ref().unwrap();
    let active_workspace_id = workspace_manager.active_workspace_id().await.unwrap();
    assert_eq!(active_workspace_id, id);
    assert_eq!(workspace_guard.manifest().await.name, new_workspace_name);

    // Check updating known_workspaces
    let list_workspaces_output = workspace_manager.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces_output.len(), 1);
    assert_eq!(list_workspaces_output[0].id, id);
    assert_eq!(list_workspaces_output[0].display_name, new_workspace_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_empty_name() {
    let (ctx, _, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let old_workspace_name = random_workspace_name();
    workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: old_workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            },
        )
        .await
        .unwrap();

    let new_workspace_name = "";
    let update_workspace_result = workspace_manager
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                name: Some(new_workspace_name.to_string()),
            },
        )
        .await;

    assert!(update_workspace_result.is_err());
    assert!(matches!(
        update_workspace_result,
        Err(OperationError::InvalidInput(_))
    ));

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_unchanged() {
    let (ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let create_workspace_output = workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            },
        )
        .await
        .unwrap();
    let id = create_workspace_output.id;

    // Rename to same name
    let update_workspace_result = workspace_manager
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                name: Some(workspace_name.clone()),
            },
        )
        .await;

    // This should be a no-op
    assert!(update_workspace_result.is_ok());

    // Check active workspace unchanged
    let active_workspace = workspace_manager.active_workspace().await;
    let (_workspace_guard, _context) = active_workspace.as_ref().unwrap();
    let active_workspace_id = workspace_manager.active_workspace_id().await.unwrap();
    assert_eq!(active_workspace_id, id);

    // Check known_workspaces unchanged
    let list_workspaces_output = workspace_manager.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces_output.len(), 1);
    assert_eq!(list_workspaces_output[0].id, id);
    assert_eq!(list_workspaces_output[0].display_name, workspace_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_not_opened() {
    let (ctx, _, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    // Try renaming a workspace with a non-existent ID
    let update_workspace_result = workspace_manager
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                name: Some(random_workspace_name()),
            },
        )
        .await;

    assert!(update_workspace_result.is_err());
    assert!(matches!(
        update_workspace_result,
        Err(OperationError::FailedPrecondition { .. })
    ));

    cleanup().await;
}
