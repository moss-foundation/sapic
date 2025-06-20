pub mod shared;

use moss_common::api::OperationError;
use moss_storage::storage::operations::{GetItem, ListByPrefix};
use moss_testutils::random_name::random_workspace_name;
use moss_workbench::models::operations::{CreateWorkspaceInput, OpenWorkspaceInput};
use moss_workspace::models::types::WorkspaceMode;
use uuid::Uuid;

use crate::shared::{setup_test_workspace_manager, workspace_key};

#[tokio::test]
async fn open_workspace_success() {
    let (ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let first_name = random_workspace_name();
    let first_output = workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: first_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await
        .unwrap();

    let second_name = random_workspace_name();
    workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: second_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await
        .unwrap();

    // Open the first workspace
    let open_result = workspace_manager
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: first_output.id,
            },
        )
        .await;
    let open_output = open_result.unwrap();

    assert_eq!(open_output.id, first_output.id);
    assert_eq!(open_output.abs_path, first_output.abs_path);

    // Check active workspace
    let active_workspace = workspace_manager.active_workspace().await;
    let (workspace_guard, _context) = active_workspace.as_ref().unwrap();
    let active_workspace_id = workspace_manager.active_workspace_id().await.unwrap();
    assert_eq!(active_workspace_id, first_output.id);
    assert_eq!(workspace_guard.manifest().await.name, first_name);

    // Check database creating first workspace entry
    let item_store = workspace_manager.__storage().item_store();
    let _ = GetItem::get(item_store.as_ref(), workspace_key(first_output.id)).unwrap();

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_not_found() {
    let (ctx, _, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let open_result = workspace_manager
        .open_workspace(&ctx, &OpenWorkspaceInput { id: Uuid::new_v4() })
        .await;
    assert!(open_result.is_err());
    assert!(matches!(open_result, Err(OperationError::NotFound { .. })));

    assert!(
        workspace_manager
            .active_workspace()
            .await
            .as_ref()
            .is_none()
    );

    // Check database not creating any entry
    let item_store = workspace_manager.__storage().item_store();
    let list_result = ListByPrefix::list_by_prefix(item_store.as_ref(), "workspace").unwrap();
    assert!(list_result.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_already_active() {
    let (ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let create_output = workspace_manager
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

    // Try to open the same workspace again
    let open_result = workspace_manager
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;
    let open_output = open_result.unwrap();

    assert_eq!(open_output.id, create_output.id);
    assert_eq!(open_output.abs_path, create_output.abs_path);

    // Check active workspace is still the same
    let active_workspace = workspace_manager.active_workspace().await;
    let (workspace_guard, _context) = active_workspace.as_ref().unwrap();
    let active_workspace_id = workspace_manager.active_workspace_id().await.unwrap();
    assert_eq!(active_workspace_id, create_output.id);
    assert_eq!(workspace_guard.manifest().await.name, workspace_name);

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_directory_deleted() {
    let (ctx, _workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let create_output = workspace_manager
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: random_workspace_name(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await
        .unwrap();

    // Delete the workspace directory
    tokio::fs::remove_dir_all(&create_output.abs_path)
        .await
        .unwrap();

    // Try to open the deleted workspace
    let open_result = workspace_manager
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;
    assert!(open_result.is_err());
    assert!(matches!(open_result, Err(OperationError::NotFound { .. })));

    assert!(
        workspace_manager
            .active_workspace()
            .await
            .as_ref()
            .is_none()
    );

    // Check database not creating any entry
    let item_store = workspace_manager.__storage().item_store();
    let list_result = ListByPrefix::list_by_prefix(item_store.as_ref(), "workspace").unwrap();
    assert!(list_result.is_empty());

    cleanup().await;
}
