mod shared;

use moss_testutils::random_name::random_collection_name;
use moss_workspace::models::operations::{CreateCollectionInput, DeleteCollectionInput};

use crate::shared::{ITEMS_KEY, setup_test_workspace};

#[tokio::test]
async fn delete_collection_success() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
            order: None,
        })
        .await
        .unwrap();

    let id = create_collection_output.id;
    let delete_collection_result = workspace
        .delete_collection(DeleteCollectionInput { id })
        .await;
    assert!(delete_collection_result.is_ok());

    // Check updating collections
    let collections = workspace.collections().await.unwrap().read().await;
    assert!(collections.is_empty());

    // Check updating database
    let dumped = workspace._storage().dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    assert_eq!(items_dump.as_object().unwrap().len(), 0);

    cleanup().await;
}

#[tokio::test]
async fn delete_collection_nonexistent_id() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let id = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
            order: None,
        })
        .await
        .unwrap()
        .id;

    workspace
        .delete_collection(DeleteCollectionInput { id })
        .await
        .unwrap();

    // Delete the collection again
    let delete_collection_result = workspace
        .delete_collection(DeleteCollectionInput { id })
        .await;

    assert!(delete_collection_result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn delete_collection_fs_already_deleted() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
            order: None,
        })
        .await
        .unwrap();

    // Delete the collection manually from the filesystem
    tokio::fs::remove_dir_all(&*create_collection_output.abs_path)
        .await
        .unwrap();

    // Even though filesystem is already deleted, deletion should succeed
    let delete_collection_result = workspace
        .delete_collection(DeleteCollectionInput {
            id: create_collection_output.id,
        })
        .await;
    assert!(delete_collection_result.is_ok());

    // Check collections are updated
    let collections = workspace.collections().await.unwrap().read().await;
    assert!(collections.is_empty());

    // TODO: Check database after implementing self-healing mechanism?

    cleanup().await;
}
