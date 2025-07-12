#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_storage::storage::operations::{GetItem, ListByPrefix};
use moss_testutils::random_name::random_collection_name;
use moss_workspace::{
    models::{
        operations::{CreateCollectionInput, DeleteCollectionInput},
        primitives::CollectionId,
    },
    services::storage_service::impl_for_integration_test::StorageServiceForIntegrationTest,
    storage::segments::{SEGKEY_COLLECTION, SEGKEY_EXPANDED_ITEMS},
};
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn delete_collection_success() {
    let (_ctx, _workspace_path, workspace, services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(&CreateCollectionInput {
            name: collection_name.clone(),
            order: 0,
            external_path: None,
            repo: None,
            icon_path: None,
        })
        .await
        .unwrap();

    let id = create_collection_output.id;
    let _ = workspace
        .delete_collection(&DeleteCollectionInput { id: id.clone() })
        .await
        .unwrap();

    // Check updating collections
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_collections(channel).await.unwrap();
    assert_eq!(output.total_returned, 0);

    // Check updating database - collection metadata should be removed
    let storage_service = services.get::<StorageServiceForIntegrationTest>();
    let item_store = storage_service.storage().item_store();

    // Check that collection-specific entries are removed
    let collection_prefix = SEGKEY_COLLECTION.join(&id.to_string());
    let list_result =
        ListByPrefix::list_by_prefix(item_store.as_ref(), &collection_prefix.to_string()).unwrap();
    assert!(list_result.is_empty());

    // Check that expanded_items no longer contains the deleted collection
    let expanded_items_value =
        GetItem::get(item_store.as_ref(), SEGKEY_EXPANDED_ITEMS.to_segkey_buf()).unwrap();
    let expanded_items: Vec<CollectionId> = expanded_items_value.deserialize().unwrap();
    assert!(!expanded_items.contains(&id));

    cleanup().await;
}

#[tokio::test]
async fn delete_collection_nonexistent_id() {
    let (_ctx, _workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let id = workspace
        .create_collection(&CreateCollectionInput {
            name: collection_name.clone(),
            order: 0,
            external_path: None,
            repo: None,
            icon_path: None,
        })
        .await
        .unwrap()
        .id;

    workspace
        .delete_collection(&DeleteCollectionInput { id: id.clone() })
        .await
        .unwrap();

    // Delete the collection again - should succeed but return None abs_path
    let delete_collection_result = workspace
        .delete_collection(&DeleteCollectionInput { id: id.clone() })
        .await
        .unwrap();

    // The second deletion should succeed but indicate nothing was deleted
    assert!(delete_collection_result.abs_path.is_none());

    cleanup().await;
}

#[tokio::test]
async fn delete_collection_fs_already_deleted() {
    let (_ctx, _workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(&CreateCollectionInput {
            name: collection_name.clone(),
            order: 0,
            external_path: None,
            repo: None,
            icon_path: None,
        })
        .await
        .unwrap();

    // Delete the collection manually from the filesystem
    tokio::fs::remove_dir_all(&*create_collection_output.abs_path)
        .await
        .unwrap();

    // Even though filesystem is already deleted, deletion should succeed
    let _ = workspace
        .delete_collection(&DeleteCollectionInput {
            id: create_collection_output.id,
        })
        .await
        .unwrap();

    // Check collections are updated
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_collections(channel).await.unwrap();
    assert_eq!(output.total_returned, 0);

    // TODO: Check database after implementing self-healing mechanism?

    cleanup().await;
}
