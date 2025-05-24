mod shared;

use moss_common::api::OperationError;
use moss_testutils::random_name::random_workspace_name;
use moss_workbench::models::operations::{CreateWorkspaceInput, OpenWorkspaceInput};
use moss_workspace::models::types::WorkspaceMode;

use crate::shared::{ITEMS_KEY, setup_test_workspace_manager, workspace_key};

#[tokio::test]
async fn open_workspace_success() {
    let (_workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let first_name = random_workspace_name();
    let first_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: first_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await
        .unwrap();

    let second_name = random_workspace_name();
    workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: second_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await
        .unwrap();

    // Open the first workspace
    let open_result = workspace_manager
        .open_workspace(&OpenWorkspaceInput {
            name: first_name.clone(),
        })
        .await;
    assert!(open_result.is_ok());
    let open_output = open_result.unwrap();

    assert_eq!(open_output.id, first_output.id);
    assert_eq!(open_output.abs_path, first_output.abs_path);

    // Check active workspace
    let active_workspace = workspace_manager.active_workspace().unwrap();
    assert_eq!(active_workspace.id, first_output.id);
    assert_eq!(active_workspace.manifest().await.name, first_name);

    // Check database creating first workspace entry
    let global_storage = workspace_manager.global_storage();
    let dumped = global_storage.dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();

    assert!(items_dump.get(workspace_key(first_output.id)).is_some());

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_not_found() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let open_result = workspace_manager
        .open_workspace(&OpenWorkspaceInput {
            name: "nonexistent_workspace_name".to_string(),
        })
        .await;
    assert!(open_result.is_err());
    assert!(matches!(open_result, Err(OperationError::NotFound { .. })));

    assert!(workspace_manager.active_workspace().is_none());

    // Check database not creating any entry
    let global_storage = workspace_manager.global_storage();
    let dumped = global_storage.dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    assert!(items_dump.as_object().unwrap().is_empty());

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_already_active() {
    let (_workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let create_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await
        .unwrap();

    // Try to open the same workspace again
    let open_result = workspace_manager
        .open_workspace(&OpenWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await;
    assert!(open_result.is_ok());
    let open_output = open_result.unwrap();

    assert_eq!(open_output.id, create_output.id);
    assert_eq!(open_output.abs_path, create_output.abs_path);

    // Check active workspace is still the same
    let active_workspace = workspace_manager.active_workspace().unwrap();
    assert_eq!(active_workspace.id, create_output.id);
    assert_eq!(active_workspace.manifest().await.name, workspace_name);

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_directory_deleted() {
    let (_workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let create_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await
        .unwrap();

    // Delete the workspace directory
    tokio::fs::remove_dir_all(&create_output.abs_path)
        .await
        .unwrap();

    // Try to open the deleted workspace
    let open_result = workspace_manager
        .open_workspace(&OpenWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await;
    assert!(open_result.is_err());
    assert!(matches!(open_result, Err(OperationError::NotFound { .. })));

    assert!(workspace_manager.active_workspace().is_none());

    // Check database not creating any entry
    let global_storage = workspace_manager.global_storage();
    let dumped = global_storage.dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    assert!(items_dump.as_object().unwrap().is_empty());

    cleanup().await;
}
