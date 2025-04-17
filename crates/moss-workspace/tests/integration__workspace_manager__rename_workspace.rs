mod shared;

use moss_fs::utils::encode_name;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_workspace_name};
use moss_workspace::models::operations::{CreateWorkspaceInput, RenameWorkspaceInput};
use moss_workspace::models::types::WorkspaceInfo;
use moss_workspace::workspace_manager::OperationError;

use crate::shared::setup_test_workspace_manager;

#[tokio::test]
async fn rename_workspace_success() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let old_workspace_name = random_workspace_name();
    let old_path = workspaces_path.join(&old_workspace_name);
    let create_workspace_output = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: old_workspace_name.clone(),
        })
        .await
        .unwrap();
    let key = create_workspace_output.key;

    let new_workspace_name = random_workspace_name();
    let rename_workspace_result = workspace_manager
        .rename_workspace(RenameWorkspaceInput {
            key,
            new_name: new_workspace_name.clone(),
        })
        .await;
    assert!(rename_workspace_result.is_ok());

    // Check filesystem rename
    let expected_path = workspaces_path.join(&new_workspace_name);
    assert!(expected_path.exists());
    assert!(!old_path.exists());

    // Check updating current workspace
    let current_workspace = workspace_manager.current_workspace().unwrap();
    assert_eq!(current_workspace.0, key);
    assert_eq!(current_workspace.1.path(), expected_path);

    // Check updating known_workspaces
    let list_workspaces_output = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(list_workspaces_output.0.len(), 1);
    assert_eq!(
        list_workspaces_output.0[0],
        WorkspaceInfo {
            path: expected_path.clone(),
            name: new_workspace_name
        }
    );

    // Clean up
    {
        tokio::fs::remove_dir_all(workspaces_path).await.unwrap();
    }
}

#[tokio::test]
async fn rename_workspace_empty_name() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let old_workspace_name = random_workspace_name();
    let create_workspace_output = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: old_workspace_name.clone(),
        })
        .await
        .unwrap();
    let key = create_workspace_output.key;

    let new_workspace_name = "".to_string();
    let rename_workspace_result = workspace_manager
        .rename_workspace(RenameWorkspaceInput {
            key,
            new_name: new_workspace_name.clone(),
        })
        .await;

    assert!(matches!(
        rename_workspace_result,
        Err(OperationError::Validation(_))
    ));

    // Clean up
    {
        tokio::fs::remove_dir_all(workspaces_path).await.unwrap();
    }
}

#[tokio::test]
async fn rename_workspace_unchanged() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let old_workspace_name = random_workspace_name();
    let create_workspace_output = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: old_workspace_name.clone(),
        })
        .await
        .unwrap();
    let key = create_workspace_output.key;

    let new_workspace_name = old_workspace_name;
    let rename_workspace_result = workspace_manager
        .rename_workspace(RenameWorkspaceInput {
            key,
            new_name: new_workspace_name.clone(),
        })
        .await;

    // This should be a no-op
    assert!(rename_workspace_result.is_ok());

    let expected_path = workspaces_path.join(&new_workspace_name);
    // Check current workspace unchanged
    let current_workspace = workspace_manager.current_workspace().unwrap();
    assert_eq!(current_workspace.0, key);
    assert_eq!(current_workspace.1.path(), expected_path);

    // Check known_workspaces unchanged
    let workspaces_list = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(workspaces_list.0.len(), 1);
    assert_eq!(
        workspaces_list.0[0],
        WorkspaceInfo {
            path: expected_path.clone(),
            name: new_workspace_name
        }
    );

    // Clean up
    {
        tokio::fs::remove_dir_all(workspaces_path).await.unwrap();
    }
}

#[tokio::test]
async fn rename_workspace_already_exists() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let existing_workspace_name = random_workspace_name();

    // Create an existing workspace
    workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: existing_workspace_name.clone(),
        })
        .await
        .unwrap();

    let new_workspace_name = random_workspace_name();
    // Create a workspace to test renaming
    let create_workspace_output = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: new_workspace_name.clone(),
        })
        .await
        .unwrap();
    let key = create_workspace_output.key;

    // Try renaming the new workspace to an existing workspace name
    let rename_workspace_result = workspace_manager
        .rename_workspace(RenameWorkspaceInput {
            key,
            new_name: existing_workspace_name.clone(),
        })
        .await;
    assert!(matches!(
        rename_workspace_result,
        Err(OperationError::AlreadyExists { .. })
    ));

    // Clean up
    {
        tokio::fs::remove_dir_all(workspaces_path).await.unwrap();
    }
}

#[tokio::test]
async fn rename_workspace_special_chars() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let create_workspace_output = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await
        .unwrap();
    let key = create_workspace_output.key;

    for char in FILENAME_SPECIAL_CHARS {
        let new_workspace_name = format!("{workspace_name}{char}");
        let expected_path = workspaces_path.join(encode_name(&new_workspace_name));
        workspace_manager
            .rename_workspace(RenameWorkspaceInput {
                key,
                new_name: new_workspace_name.clone(),
            })
            .await
            .unwrap();

        // Check updating current workspace
        let current_workspace = workspace_manager.current_workspace().unwrap();
        assert_eq!(current_workspace.0, key);
        assert_eq!(current_workspace.1.path(), expected_path);

        // Checking updating known_workspaces
        let list_workspaces_output = workspace_manager.list_workspaces().await.unwrap();
        assert_eq!(list_workspaces_output.0.len(), 1);
        assert_eq!(
            list_workspaces_output.0[0],
            WorkspaceInfo {
                path: expected_path.clone(),
                name: new_workspace_name
            }
        );
    }

    // Clean up
    {
        tokio::fs::remove_dir_all(workspaces_path).await.unwrap();
    }
}
