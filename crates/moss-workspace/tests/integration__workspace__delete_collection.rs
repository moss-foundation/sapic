mod shared;

use moss_testutils::random_name::random_collection_name;
use moss_workspace::models::operations::{CreateCollectionInput, DeleteCollectionInput};

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn delete_collection_success() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let key = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await
        .unwrap()
        .key;
    let delete_collection_result = workspace
        .delete_collection(DeleteCollectionInput { key })
        .await;
    assert!(delete_collection_result.is_ok());

    // Check updating collections
    let describe_output = workspace.describe().await.unwrap();
    assert!(describe_output.collections.is_empty());

    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_collection_nonexistent_key() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let key = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await
        .unwrap()
        .key;

    workspace
        .delete_collection(DeleteCollectionInput { key })
        .await
        .unwrap();

    // Delete the collection again
    let delete_collection_result = workspace
        .delete_collection(DeleteCollectionInput { key })
        .await;

    assert!(delete_collection_result.is_err());

    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_collection_fs_already_deleted() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await
        .unwrap();

    // Delete the collection
    tokio::fs::remove_dir_all(create_collection_output.path)
        .await
        .unwrap();

    let result = workspace
        .delete_collection(DeleteCollectionInput {
            key: create_collection_output.key,
        })
        .await;
    assert!(result.is_ok());

    // Check updating collections
    let describe_output = workspace.describe().await.unwrap();
    assert!(describe_output.collections.is_empty());

    {
        std::fs::remove_dir_all(workspace_path).unwrap();
    }
}
