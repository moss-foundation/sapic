mod shared;

use moss_fs::utils::encode_name;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_workspace_name};
use moss_workspace::models::operations::CreateWorkspaceInput;
use moss_workspace::workspace_manager::*;

use crate::shared::setup_test_workspace_manager;

#[tokio::test]
async fn create_workspace_success() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);
    let create_workspace_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await;
    assert!(create_workspace_result.is_ok());
    assert!(expected_path.exists());

    let create_workspace_output = create_workspace_result.unwrap();

    // Check updating current workspace
    let current_workspace = workspace_manager.current_workspace().unwrap();
    assert_eq!(current_workspace.0, create_workspace_output.key);
    assert_eq!(current_workspace.1.path(), expected_path);

    // Check updating known_workspaces
    let list_workspace_output = workspace_manager.list_workspaces().await.unwrap();

    assert_eq!(list_workspace_output.0.len(), 1);
    assert_eq!(list_workspace_output.0[0].name, workspace_name);
    assert_eq!(list_workspace_output.0[0].path, expected_path);

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_workspace_empty_name() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let create_workspace_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: "".to_string(),
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
        })
        .await
        .unwrap();

    // Try to create workspace with same name
    let create_workspace_result = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: workspace_name.clone(),
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

    let workspace_name_list = FILENAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_workspace_name()))
        .collect::<Vec<String>>();

    for name in workspace_name_list {
        let expected_path = workspaces_path.join(encode_name(&name));
        let create_workspace_output = workspace_manager
            .create_workspace(&CreateWorkspaceInput { name: name.clone() })
            .await
            .unwrap();

        assert!(expected_path.exists());
        // Check updating current workspace
        let current_workspace = workspace_manager.current_workspace().unwrap();
        assert_eq!(current_workspace.0, create_workspace_output.key);
        assert_eq!(current_workspace.1.path(), expected_path);

        // Check updating known_workspaces
        let workspaces_list = workspace_manager.list_workspaces().await.unwrap();
        let matching_workspace = workspaces_list
            .0
            .iter()
            .find(|info| info.name == name)
            .expect("Workspace should exist in the list");

        assert_eq!(matching_workspace.path, expected_path);
        assert_eq!(matching_workspace.name, name);
    }

    // Clean up
    cleanup().await;
}
