use moss_workspace::models::operations::{CreateCollectionInput, DeleteCollectionInput};
use crate::shared::{random_collection_name, setup_test_workspace};

mod shared;

#[tokio::test]
async fn delete_collection_success() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let expected_path = workspace_path.join(&collection_name);
    let key = workspace.create_collection(
        CreateCollectionInput {
            name: collection_name.clone(),
        }
    ).await.unwrap().key;
    let result = workspace.delete_collection(
        DeleteCollectionInput {
            key
        }
    ).await;
    assert!(result.is_ok());

    // Check folder is removed
    assert!(!expected_path.exists());

    // Check updating collections
    let collections = workspace.list_collections().await.unwrap();
    assert!(collections.0.is_empty());

    {
        std::fs::remove_dir_all(workspace_path).unwrap();
    }
}

#[tokio::test]
async fn delete_collection_nonexistent_key() {
    // FIXME: Should this be an error or a no-op, since technically what needs to be deleted is gone?
    // This might happen, e.g., when the frontend tries to delete already deleted collection
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let key = workspace.create_collection(
        CreateCollectionInput {
            name: collection_name.clone(),
        }
    ).await.unwrap().key;
    workspace.delete_collection(
        DeleteCollectionInput {
            key
        }
    ).await.unwrap();
    let result = workspace.delete_collection(
        DeleteCollectionInput {
            key
        }
    ).await;
    assert!(result.is_err());

    {
        std::fs::remove_dir_all(workspace_path).unwrap();
    }
}

#[tokio::test]
async fn delete_collection_fs_already_deleted() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let expected_path = workspace_path.join(&collection_name);
    let key = workspace.create_collection(
        CreateCollectionInput {
            name: collection_name.clone(),
        }
    ).await.unwrap().key;

    // Delete the collection
    std::fs::remove_dir_all(expected_path).unwrap();

    let result = workspace.delete_collection(
        DeleteCollectionInput {
            key
        }
    ).await;
    assert!(result.is_ok());

    // Check updating collections
    let collections = workspace.list_collections().await.unwrap();
    assert!(collections.0.is_empty());

    {
        std::fs::remove_dir_all(workspace_path).unwrap();
    }
}