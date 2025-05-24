mod shared;

use moss_common::api::OperationError;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_workspace_name};
use moss_workbench::models::operations::CreateWorkspaceInput;
use moss_workspace::models::types::WorkspaceMode;
use std::{path::Path, sync::Arc};

use crate::shared::{ITEMS_KEY, setup_test_workspace_manager, workspace_key};

#[tokio::test]
async fn create_workspace_success() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let create_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await;
    assert!(create_result.is_ok());

    let create_output = create_result.unwrap();
    let expected_path: Arc<Path> = workspaces_path.join(&create_output.id.to_string()).into();

    assert!(expected_path.exists());

    // Check active workspace
    let active_workspace = workspace_manager.active_workspace().unwrap();
    assert_eq!(active_workspace.id, create_output.id);
    assert_eq!(active_workspace.abs_path(), &expected_path);
    assert_eq!(active_workspace.manifest().await.name, workspace_name);

    // Check known_workspaces
    let list_workspaces = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, create_output.id);
    assert_eq!(list_workspaces[0].display_name, workspace_name);

    // Check database
    let global_storage = workspace_manager._global_storage();
    let dumped = global_storage.dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    assert!(items_dump.get(workspace_key(create_output.id)).is_some());

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_empty_name() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let create_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: "".to_string(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await;

    assert!(matches!(
        create_result,
        Err(OperationError::InvalidInput(_))
    ));

    // Ensure no workspace was created or activated
    let list_workspaces = workspace_manager.list_workspaces().await.unwrap();
    assert!(list_workspaces.is_empty());
    assert!(workspace_manager.active_workspace().is_none());

    // Check database
    let global_storage = workspace_manager._global_storage();
    let dumped = global_storage.dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    assert!(items_dump.as_object().unwrap().is_empty());

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_same_name() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();

    // Create first workspace
    let first_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await;
    assert!(first_result.is_ok());
    let first_output = first_result.unwrap();

    let first_path: Arc<Path> = workspaces_path.join(&first_output.id.to_string()).into();
    assert!(first_path.exists());

    // Check first workspace is in list
    let list_after_first = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(list_after_first.len(), 1);
    assert_eq!(list_after_first[0].id, first_output.id);
    assert_eq!(list_after_first[0].display_name, workspace_name);

    // Create second workspace with same name
    let second_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await;
    assert!(second_result.is_ok());
    let second_output = second_result.unwrap();

    let second_path: Arc<Path> = workspaces_path.join(&second_output.id.to_string()).into();
    assert!(second_path.exists());
    assert_ne!(first_output.id, second_output.id);

    // Check active workspace is the second one
    let active_workspace = workspace_manager.active_workspace().unwrap();
    assert_eq!(active_workspace.id, second_output.id);
    assert_eq!(active_workspace.abs_path(), &second_path);
    assert_eq!(active_workspace.manifest().await.name, workspace_name);

    // Check both workspaces are in list
    let list_after_second = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(list_after_second.len(), 2);

    let listed_first = list_after_second
        .iter()
        .find(|w| w.id == first_output.id)
        .unwrap();
    assert_eq!(listed_first.display_name, workspace_name);

    let listed_second = list_after_second
        .iter()
        .find(|w| w.id == second_output.id)
        .unwrap();
    assert_eq!(listed_second.display_name, workspace_name);

    // Check only second workspace has entry in the databased since it's been opened
    // Check database
    let global_storage = workspace_manager._global_storage();
    let dumped = global_storage.dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    assert!(items_dump.get(workspace_key(second_output.id)).is_some());
    assert!(items_dump.get(workspace_key(first_output.id)).is_none());

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_special_chars() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let base_name = random_workspace_name();
    let mut created_count = 0;

    for special_char in FILENAME_SPECIAL_CHARS.iter() {
        let name = format!("{}{}", base_name, special_char);

        let create_result = workspace_manager
            .create_workspace(&CreateWorkspaceInput {
                name: name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            })
            .await;
        assert!(create_result.is_ok());
        let create_output = create_result.unwrap();
        created_count += 1;

        let expected_path: Arc<Path> = workspaces_path.join(&create_output.id.to_string()).into();
        assert!(expected_path.exists());

        // Check active workspace
        let active_workspace = workspace_manager.active_workspace().unwrap();
        assert_eq!(active_workspace.id, create_output.id);
        assert_eq!(active_workspace.abs_path(), &expected_path);
        assert_eq!(active_workspace.manifest().await.name, name);

        // Check workspace is in list
        let list_workspaces = workspace_manager.list_workspaces().await.unwrap();
        assert_eq!(list_workspaces.len(), created_count);

        let matching_workspace = list_workspaces
            .iter()
            .find(|w| w.id == create_output.id)
            .unwrap();
        assert_eq!(matching_workspace.display_name, name);
    }

    // Check database
    let global_storage = workspace_manager._global_storage();
    let dumped = global_storage.dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    assert_eq!(items_dump.as_object().unwrap().len(), created_count);

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_not_open_on_creation() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let create_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await;
    assert!(create_result.is_ok());
    let create_output = create_result.unwrap();

    let expected_path: Arc<Path> = workspaces_path.join(&create_output.id.to_string()).into();
    assert!(expected_path.exists());

    // Check that no workspace is active
    assert!(workspace_manager.active_workspace().is_none());

    // Check workspace is in list
    let list_workspaces = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, create_output.id);
    assert_eq!(list_workspaces[0].display_name, workspace_name);

    // Check that a database entry is not created
    let global_storage = workspace_manager._global_storage();
    let dumped = global_storage.dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    assert!(items_dump.as_object().unwrap().is_empty());
    cleanup().await;
}
