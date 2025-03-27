mod shared;

use moss_fs::utils::encode_directory_name;
use shared::random_workspace_name;

use moss_workspace::models::operations::CreateWorkspaceInput;
use moss_workspace::models::types::WorkspaceInfo;
use moss_workspace::workspace_manager::*;
use crate::shared::{setup_test_workspace_manager, SPECIAL_CHARS};

#[tokio::test]
async fn create_workspace_success() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);
    let output = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await.unwrap();

    assert!(expected_path.exists());

    // Check updating current workspace
    let current_workspace = workspace_manager.current_workspace().unwrap();
    assert_eq!(current_workspace.0.as_u64(), output.key);
    assert_eq!(current_workspace.1.path(), expected_path);

    // Check updating known_workspaces
    let workspaces_list = workspace_manager.list_workspaces().await.unwrap();
    assert_eq!(workspaces_list.0.len(), 1);
    assert_eq!(workspaces_list.0[0], WorkspaceInfo {path: expected_path.clone(), name: workspace_name} );

    // Clean up
    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }
}

#[tokio::test]
async fn create_workspace_empty_name() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let result = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: "".to_string(),
        })
        .await;

    assert!(matches!(result, Err(OperationError::Validation(_))));

    // Clean up
    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }
}

#[tokio::test]
async fn create_workspace_already_exists() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let workspace_name = random_workspace_name();
    let expected_path = workspaces_path.join(&workspace_name);

    // Create first workspace
    workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await.unwrap();

    // Try to create workspace with same name
    let result = workspace_manager
        .create_workspace(CreateWorkspaceInput {
            name: workspace_name.clone(),
        })
        .await;

    match result {
        Err(OperationError::AlreadyExists { name, path }) => {
            assert_eq!(name, workspace_name);
            assert_eq!(path, expected_path);
        }
        _ => panic!("Expected AlreadyExists error"),
    }

    // Clean up
    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }
}

// #[tokio::test]
// async fn create_workspace_invalid_path() {
//     let fs = Arc::new(DiskFileSystem::new());
//     let workspaces_path: PathBuf = PathBuf::from("/nonexistent/path");
//
//     let workspace_manager = WorkspaceManager::new(fs, workspaces_path.clone());
//
//     let result = workspace_manager
//         .create_workspace(CreateWorkspaceInput {
//             name: random_workspace_name(),
//         })
//         .await;
//
//     assert!(matches!(result, Err(OperationError::Unknown(_))));
// }

#[tokio::test]
async fn create_workspace_special_chars() {
    let (workspaces_path, workspace_manager) = setup_test_workspace_manager().await;

    let workspace_name_list = SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_workspace_name()))
        .collect::<Vec<String>>();

    for name in workspace_name_list {
        let expected_path = workspaces_path.join(encode_directory_name(&name));
        let output = workspace_manager.create_workspace( CreateWorkspaceInput {
            name: name.clone()
        }).await.unwrap();

        assert!(expected_path.exists());
        // Check updating current workspace
        let current_workspace = workspace_manager.current_workspace().unwrap();
        assert_eq!(current_workspace.0.as_u64(), output.key);
        assert_eq!(current_workspace.1.path(), expected_path);

        // Check updating known_workspaces
        let workspaces_list = workspace_manager.list_workspaces().await.unwrap();
        assert!(workspaces_list.0.iter().any(|info| info == &WorkspaceInfo {
            name: name.clone(),
            path: expected_path.clone()
        }));
    }
    // Clean up
    {
        std::fs::remove_dir_all(workspaces_path).unwrap();
    }

}
