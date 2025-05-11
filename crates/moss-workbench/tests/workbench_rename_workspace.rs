mod shared;

use moss_common::api::OperationError;
use moss_fs::utils::encode_name;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_workspace_name};
use moss_workbench::models::operations::{CreateWorkspaceInput, UpdateWorkspaceInput};
use moss_workspace::models::types::WorkspaceMode;
use std::{path::Path, sync::Arc};

use crate::shared::setup_test_workspace_manager;

#[tokio::test]
async fn rename_workspace_success() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let old_workspace_name = random_workspace_name();
    let old_path: Arc<Path> = workspaces_path.join(&old_workspace_name).into();
    let create_workspace_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: old_workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await
        .unwrap();
    let id = create_workspace_output.id;

    let new_workspace_name = random_workspace_name();
    let rename_workspace_result = workspace_manager
        .update_workspace(UpdateWorkspaceInput {
            id,
            name: Some(new_workspace_name.clone()),
        })
        .await;
    assert!(rename_workspace_result.is_ok());

    // Check filesystem rename
    let expected_path: Arc<Path> = workspaces_path.join(&new_workspace_name).into();
    assert!(expected_path.exists());
    assert!(!old_path.exists());

    // Check updating active workspace
    let active_workspace = workspace_manager.active_workspace().unwrap();
    assert_eq!(active_workspace.id, id);

    // Check updating known_workspaces
    let list_workspaces_output = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(list_workspaces_output.len(), 1);
    assert_eq!(list_workspaces_output[0].id, id);
    assert_eq!(list_workspaces_output[0].display_name, new_workspace_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_empty_name() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let old_workspace_name = random_workspace_name();
    let create_workspace_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: old_workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await
        .unwrap();
    let id = create_workspace_output.id;

    let new_workspace_name = "";
    let rename_workspace_result = workspace_manager
        .update_workspace(UpdateWorkspaceInput {
            id,
            name: Some(new_workspace_name.to_string()),
        })
        .await;

    assert!(rename_workspace_result.is_err());
    assert!(matches!(
        rename_workspace_result,
        Err(OperationError::Validation(_))
    ));

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_unchanged() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

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

    // Rename to same name
    let rename_workspace_result = workspace_manager
        .update_workspace(UpdateWorkspaceInput {
            id,
            name: Some(workspace_name.clone()),
        })
        .await;

    // This should be a no-op
    assert!(rename_workspace_result.is_ok());

    // Check active workspace unchanged
    let active_workspace = workspace_manager.active_workspace().unwrap();
    assert_eq!(active_workspace.id, id);

    // Check known_workspaces unchanged
    let workspaces_list = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(workspaces_list.len(), 1);
    assert_eq!(workspaces_list[0].id, id);
    assert_eq!(workspaces_list[0].display_name, workspace_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_already_exists() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    let existing_workspace_name = random_workspace_name();

    // Create an existing workspace
    workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: existing_workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: false,
        })
        .await
        .unwrap();

    let new_workspace_name = random_workspace_name();
    // Create a workspace to test renaming
    let create_workspace_output = workspace_manager
        .create_workspace(&CreateWorkspaceInput {
            name: new_workspace_name.clone(),
            mode: WorkspaceMode::default(),
            open_on_creation: true,
        })
        .await
        .unwrap();
    let id = create_workspace_output.id;

    // Try renaming the new workspace to an existing workspace name
    let rename_workspace_result = workspace_manager
        .update_workspace(UpdateWorkspaceInput {
            id,
            name: Some(existing_workspace_name.clone()),
        })
        .await;
    assert!(rename_workspace_result.is_err());
    assert!(matches!(
        rename_workspace_result,
        Err(OperationError::AlreadyExists { .. })
    ));

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_nonexistent_id() {
    let (_, workspace_manager, cleanup) = setup_test_workspace_manager().await;

    // Create a non-existent ID
    let id = moss_common::identifier::Identifier::new(&std::sync::Arc::new(
        std::sync::atomic::AtomicUsize::new(1000),
    ));

    // Try renaming a workspace with a non-existent ID
    let rename_workspace_result = workspace_manager
        .update_workspace(UpdateWorkspaceInput {
            id,
            name: Some(random_workspace_name()),
        })
        .await;

    assert!(rename_workspace_result.is_err());
    assert!(matches!(
        rename_workspace_result,
        Err(OperationError::NotFound { .. })
    ));

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_special_chars() {
    let (workspaces_path, workspace_manager, cleanup) = setup_test_workspace_manager().await;

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

    for char in FILENAME_SPECIAL_CHARS {
        let new_workspace_name = format!("{workspace_name}{char}");
        let rename_result = workspace_manager
            .update_workspace(UpdateWorkspaceInput {
                id,
                name: Some(new_workspace_name.clone()),
            })
            .await;

        assert!(rename_result.is_ok());

        // Check folder was renamed
        let expected_path: Arc<Path> = workspaces_path
            .join(&encode_name(&new_workspace_name))
            .into();
        assert!(expected_path.exists());

        // Check updating active workspace
        let active_workspace = workspace_manager.active_workspace().unwrap();
        assert_eq!(active_workspace.id, id);

        // Checking updating known_workspaces
        let list_workspaces_output = workspace_manager.list_workspaces().await.unwrap();
        assert_eq!(list_workspaces_output.len(), 1);
        assert_eq!(list_workspaces_output[0].id, id);
        assert_eq!(list_workspaces_output[0].display_name, new_workspace_name);
    }

    cleanup().await;
}
