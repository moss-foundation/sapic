mod shared;

use moss_common::api::OperationError;
use moss_fs::utils::encode_name;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_workspace_name};
use moss_workbench::models::operations::CreateWorkspaceInput;
use moss_workspace::models::types::WorkspaceMode;
use std::{path::Path, sync::Arc};

use crate::shared::setup_test_workspace_manager;

#[tokio::test]
async fn create_workspace_success() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path: Arc<Path> = workspaces_path.join(&workspace_name).into();
    let create_workspace_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await;
    assert!(create_workspace_result.is_ok());
    assert!(expected_path.exists());

    let create_workspace_output = create_workspace_result.unwrap();
    assert_eq!(create_workspace_output.abs_path, expected_path);

    // Check updating current workspace
    let active_workspace = workspace_manager.active_workspace().unwrap();
    assert_eq!(active_workspace.id, create_workspace_output.id);
    assert_eq!(active_workspace.abs_path(), &expected_path);

    // Check updating known_workspaces
    let list_workspaces_output = workspace_manager.list_workspaces().await.unwrap();

    assert_eq!(list_workspaces_output.len(), 1);
    assert_eq!(list_workspaces_output[0].id, create_workspace_output.id);

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_workspace_empty_name() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let create_workspace_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: "".to_string(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await;

    assert!(matches!(
        create_workspace_result,
        Err(OperationError::Validation(_))
    ));

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_workspace_already_exists() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);

    // Create first workspace
    workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await
        .unwrap();

    // Try to create workspace with same name
    let create_workspace_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await;

    match create_workspace_result {
        Err(OperationError::AlreadyExists { name, path }) => {
            assert_eq!(name, workspace_name);
            assert_eq!(path, expected_path);
        }
        _ => panic!("Expected AlreadyExists error"),
    }

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_workspace_special_chars() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name_list = {
        let base_name = random_workspace_name();

        FILENAME_SPECIAL_CHARS
            .into_iter()
            .map(|s| format!("{base_name}{s}"))
            .collect::<Vec<String>>()
    };

    for name in workspace_name_list {
        let encoded_name = encode_name(&name);
        let expected_path: Arc<Path> = workspaces_path.join(&encoded_name).into();
        let create_workspace_output = workspace_manager
            .create_workspace(&CreateWorkspaceInput {
                name: name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            })
            .await
            .unwrap();

        assert!(expected_path.exists());

        // Check updating current workspace
        let active_workspace = workspace_manager.active_workspace().unwrap();
        assert_eq!(active_workspace.id, create_workspace_output.id);

        // Check updating known_workspaces
        let workspaces_list = workspace_manager.list_workspaces().await.unwrap();
        let matching_workspace = workspaces_list
            .iter()
            .find(|info| info.display_name == name)
            .expect("Workspace should exist in the list");

        assert_eq!(matching_workspace.id, create_workspace_output.id);
        assert_eq!(matching_workspace.display_name, name);
    }

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_workspace_not_open_on_creation() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path: Arc<Path> = workspaces_path.join(&workspace_name).into();

    let create_workspace_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await;

    assert!(create_workspace_result.is_ok());
    assert!(expected_path.exists());

    let create_workspace_output = create_workspace_result.unwrap();

    // Check that the workspace was not set as active
    let active_workspace_result = workspace_manager.active_workspace();
    assert!(
        active_workspace_result.is_none(),
        "No workspace should be active"
    );

    // Check that the workspace is in the known workspaces list
    let list_workspaces_output = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(list_workspaces_output.len(), 1);
    assert_eq!(list_workspaces_output[0].id, create_workspace_output.id);
    assert_eq!(list_workspaces_output[0].display_name, workspace_name);

    // Clean up
    cleanup().await;
}
