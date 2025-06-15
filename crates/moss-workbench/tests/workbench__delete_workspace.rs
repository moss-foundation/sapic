pub mod shared;

use crate::shared::{setup_test_workspace_manager, workspace_key};
use moss_common::api::OperationError;
use moss_storage::storage::operations::GetItem;
use moss_testutils::random_name::random_workspace_name;
use moss_workbench::models::operations::{CreateWorkspaceInput, DeleteWorkspaceInput};
use moss_workspace::models::types::WorkspaceMode;
use std::{path::Path, sync::Arc};

#[tokio::test]
async fn delete_workspace_success() {
    let (ctx, workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path: Arc<Path> = workspaces_path.join(&workspace_name).into();
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
    let delete_workspace_result = workspace_manager
        .delete_workspace(&ctx, &DeleteWorkspaceInput { id })
        .await;
    assert!(delete_workspace_result.is_ok());

    // Check folder is removed
    assert!(!expected_path.exists());

    // Check removing active workspace
    let active_workspace = workspace_manager.active_workspace();
    assert!(active_workspace.await.is_none());

    // Check updating known_workspaces
    let list_workspaces_output = workspace_manager.list_workspaces(&ctx).await.unwrap();
    assert!(list_workspaces_output.is_empty());

    // Check removing entry from the database
    let item_store = workspace_manager.__storage().item_store();
    assert!(GetItem::get(item_store.as_ref(), workspace_key(id)).is_err());

    cleanup().await;
}

#[tokio::test]
async fn delete_workspace_nonexistent_id() {
    let (ctx, _, workspace_manager, cleanup) = setup_test_workspace_manager().await;

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

    workspace_manager
        .delete_workspace(&ctx, &DeleteWorkspaceInput { id })
        .await
        .unwrap();

    let delete_workspace_result = workspace_manager
        .delete_workspace(&ctx, &DeleteWorkspaceInput { id })
        .await;
    assert!(delete_workspace_result.is_err());
    assert!(matches!(
        delete_workspace_result,
        Err(OperationError::NotFound { .. })
    ));

    cleanup().await;
}
