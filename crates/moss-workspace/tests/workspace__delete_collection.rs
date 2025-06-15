pub mod shared;

use moss_storage::storage::operations::ListByPrefix;
use moss_testutils::random_name::random_collection_name;
use moss_workspace::models::operations::{CreateCollectionInput, DeleteCollectionInput};

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn delete_collection_success() {
    let (ctx, _workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: None,
                external_path: None,
            },
        )
        .await
        .unwrap();

    let id = create_collection_output.id;
    let delete_collection_result = workspace
        .delete_collection(&ctx, &DeleteCollectionInput { id })
        .await;
    assert!(delete_collection_result.is_ok());

    // Check updating collections
    let collections = workspace.collections(&ctx).await.unwrap().read().await;
    assert!(collections.is_empty());

    // Check updating database
    let item_store = workspace.__storage().item_store();
    let list_result = ListByPrefix::list_by_prefix(item_store.as_ref(), "collection").unwrap();
    assert!(list_result.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn delete_collection_nonexistent_id() {
    let (ctx, _workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let id = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: None,
                external_path: None,
            },
        )
        .await
        .unwrap()
        .id;

    workspace
        .delete_collection(&ctx, &DeleteCollectionInput { id })
        .await
        .unwrap();

    // Delete the collection again
    let delete_collection_result = workspace
        .delete_collection(&ctx, &DeleteCollectionInput { id })
        .await;

    assert!(delete_collection_result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn delete_collection_fs_already_deleted() {
    let (ctx, _workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: None,
                external_path: None,
            },
        )
        .await
        .unwrap();

    // Delete the collection manually from the filesystem
    tokio::fs::remove_dir_all(&*create_collection_output.abs_path)
        .await
        .unwrap();

    // Even though filesystem is already deleted, deletion should succeed
    let delete_collection_result = workspace
        .delete_collection(
            &ctx,
            &DeleteCollectionInput {
                id: create_collection_output.id,
            },
        )
        .await;
    assert!(delete_collection_result.is_ok());

    // Check collections are updated
    let collections = workspace.collections(&ctx).await.unwrap().read().await;
    assert!(collections.is_empty());

    // TODO: Check database after implementing self-healing mechanism?

    cleanup().await;
}
