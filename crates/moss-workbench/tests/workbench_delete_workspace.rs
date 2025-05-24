mod shared;

use moss_common::api::OperationError;
use moss_testutils::random_name::random_workspace_name;
use moss_workbench::models::operations::{CreateWorkspaceInput, DeleteWorkspaceInput};
use moss_workspace::models::types::WorkspaceMode;
use std::{path::Path, sync::Arc};

use crate::shared::{ITEMS_KEY, setup_test_workspace_manager};

#[tokio::test]
async fn delete_workspace_success() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path: Arc<Path> = workspaces_path.join(&workspace_name).into();
    let create_workspace_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await
        .unwrap();

    let id = create_workspace_output.id;
    let delete_workspace_result = workspace_manager
        .delete_workspace(&DeleteWorkspaceInput { id })
        .await;
    assert!(delete_workspace_result.is_ok());

    // Check folder is removed
    assert!(!expected_path.exists());

    // Check removing active workspace
    let active_workspace = workspace_manager.active_workspace();
    assert!(active_workspace.is_none());

    // Check updating known_workspaces
    let list_workspaces_output = workspace_manager.list_workspaces().await.unwrap();
    assert!(list_workspaces_output.is_empty());

    // Check updating database
    let global_storage = workspace_manager._global_storage();
    let dumped = global_storage.dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();

    assert!(items_dump.as_object().unwrap().is_empty());

    cleanup().await;
}

#[tokio::test]
async fn delete_workspace_nonexistent_id() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let create_workspace_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await
        .unwrap();
    let id = create_workspace_output.id;

    workspace_manager
        .delete_workspace(&DeleteWorkspaceInput { id })
        .await
        .unwrap();

    let delete_workspace_result = workspace_manager
        .delete_workspace(&DeleteWorkspaceInput { id })
        .await;
    assert!(delete_workspace_result.is_err());
    assert!(matches!(
        delete_workspace_result,
        Err(OperationError::NotFound { .. })
    ));

    cleanup().await;
}
