pub mod shared;

use moss_common::api::OperationError;
use moss_storage::storage::operations::{GetItem, ListByPrefix};
use moss_storage::workspace_storage::entities::collection_store_entities::CollectionCacheEntity;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_collection_name};
use moss_workspace::models::operations::CreateCollectionInput;

use crate::shared::{collection_key, setup_test_workspace};

#[tokio::test]
async fn create_collection_success() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_result = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
            order: None,
            external_path: None,
        })
        .await;

    assert!(create_collection_result.is_ok());

    let create_collection_output = create_collection_result.unwrap();
    let collections = workspace.collections().await.unwrap().read().await;

    assert_eq!(collections.len(), 1);

    // Verify the directory was created
    assert!(create_collection_output.abs_path.exists());

    // Verify the db entry was created
    let id = create_collection_output.id;

    let item_store = workspace.__storage().item_store();
    let entity: CollectionCacheEntity = GetItem::get(item_store.as_ref(), collection_key(id))
        .unwrap()
        .deserialize()
        .unwrap();
    assert_eq!(
        entity,
        CollectionCacheEntity {
            order: None,
            external_abs_path: None
        }
    );

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_collection_empty_name() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = "".to_string();
    let create_collection_result = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
            order: None,
            external_path: None,
        })
        .await;

    assert!(matches!(
        create_collection_result,
        Err(OperationError::InvalidInput(_))
    ));

    // Check that the database is empty
    let item_store = workspace.__storage().item_store();
    let list_result = ListByPrefix::list_by_prefix(item_store.as_ref(), "collection").unwrap();
    assert!(list_result.is_empty());

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_collection_special_chars() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name_list = FILENAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_collection_name()))
        .collect::<Vec<String>>();

    for collection_name in collection_name_list {
        let create_collection_result = workspace
            .create_collection(CreateCollectionInput {
                name: collection_name.clone(),
                order: None,
                external_path: None,
            })
            .await;
        assert!(create_collection_result.is_ok());

        let create_collection_output = create_collection_result.unwrap();
        let collections = workspace.collections().await.unwrap().read().await;

        assert!(collections.contains_key(&create_collection_output.id));

        // Verify the directory was created
        assert!(create_collection_output.abs_path.exists());

        // Verify the db entry was created
        let id = create_collection_output.id;
        let item_store = workspace.__storage().item_store();
        let entity: CollectionCacheEntity = GetItem::get(item_store.as_ref(), collection_key(id))
            .unwrap()
            .deserialize()
            .unwrap();
        assert_eq!(
            entity,
            CollectionCacheEntity {
                order: None,
                external_abs_path: None
            }
        );
    }

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_collection_with_order() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_result = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
            order: Some(42),
            external_path: None,
        })
        .await;

    assert!(create_collection_result.is_ok());

    let create_collection_output = create_collection_result.unwrap();
    let collections = workspace.collections().await.unwrap().read().await;

    assert_eq!(collections.len(), 1);
    // Verify the order is correctly stored
    let order = collections.iter().next().unwrap().1.read().await.order;
    assert_eq!(order, Some(42));

    // Verify the directory was created
    assert!(create_collection_output.abs_path.exists());

    // Verify the db entry was created
    let id = create_collection_output.id;
    let item_store = workspace.__storage().item_store();
    let entity: CollectionCacheEntity = GetItem::get(item_store.as_ref(), collection_key(id))
        .unwrap()
        .deserialize()
        .unwrap();
    assert_eq!(
        entity,
        CollectionCacheEntity {
            order: Some(42),
            external_abs_path: None
        }
    );

    // Clean up
    cleanup().await;
}
