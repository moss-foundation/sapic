#![cfg(feature = "integration-tests")]

pub mod shared;

use crate::shared::{generate_random_icon, setup_test_workspace};
use moss_applib::mock::MockAppRuntime;
use moss_storage::storage::operations::GetItem;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_collection_name};
use moss_workspace::{
    models::{operations::CreateCollectionInput, primitives::CollectionId},
    services::{
        AnyCollectionService, collection_service::CollectionService,
        storage_service::StorageService,
    },
    storage::segments::{SEGKEY_COLLECTION, SEGKEY_EXPANDED_ITEMS},
};
use tauri::ipc::Channel;

#[tokio::test]
async fn create_collection_success() {
    let (ctx, _workspace_path, workspace, services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: 0,
                external_path: None,
                repository: None,
                icon_path: None,
            },
        )
        .await;

    let create_collection_output = create_collection_result.unwrap();

    // Verify through stream_collections
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_collections(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the directory was created
    assert!(create_collection_output.abs_path.exists());

    // Verify the db entries were created
    let id = create_collection_output.id;
    let storage_service = services.get::<StorageService<MockAppRuntime>>();
    let item_store = storage_service.storage().item_store();

    // Check order was stored
    let order_key = SEGKEY_COLLECTION.join(&id.to_string()).join("order");
    let order_value = GetItem::get(item_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: usize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 0);

    // Check expanded_items contains the collection id
    let expanded_items_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
    )
    .await
    .unwrap();
    let expanded_items: Vec<CollectionId> = expanded_items_value.deserialize().unwrap();
    assert!(expanded_items.contains(&id));

    cleanup().await;
}

#[tokio::test]
async fn create_collection_empty_name() {
    let (ctx, _workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    let collection_name = "".to_string();
    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: 0,
                external_path: None,
                repository: None,
                icon_path: None,
            },
        )
        .await;

    assert!(create_collection_result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn create_collection_special_chars() {
    let (ctx, _workspace_path, workspace, services, cleanup) = setup_test_workspace().await;

    let collection_name_list = FILENAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_collection_name()))
        .collect::<Vec<String>>();

    for collection_name in &collection_name_list {
        let create_collection_result = workspace
            .create_collection(
                &ctx,
                &CreateCollectionInput {
                    name: collection_name.clone(),
                    order: 0,
                    external_path: None,
                    repository: None,
                    icon_path: None,
                },
            )
            .await;

        let create_collection_output = create_collection_result.unwrap();

        // Verify the directory was created
        assert!(create_collection_output.abs_path.exists());

        // Verify the db entries were created
        let id = create_collection_output.id;
        let storage_service = services.get::<StorageService<MockAppRuntime>>();
        let item_store = storage_service.storage().item_store();

        // Check order was stored
        let order_key = SEGKEY_COLLECTION.join(&id.to_string()).join("order");
        let order_value = GetItem::get(item_store.as_ref(), &ctx, order_key)
            .await
            .unwrap();
        let stored_order: usize = order_value.deserialize().unwrap();
        assert_eq!(stored_order, 0);

        // Check expanded_items contains the collection id
        let expanded_items_value = GetItem::get(
            item_store.as_ref(),
            &ctx,
            SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
        )
        .await
        .unwrap();
        let expanded_items: Vec<CollectionId> = expanded_items_value.deserialize().unwrap();
        assert!(expanded_items.contains(&id));
    }

    // Verify all collections are returned through stream_collections
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_collections(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, collection_name_list.len());

    cleanup().await;
}

#[tokio::test]
async fn create_collection_with_order() {
    let (ctx, _workspace_path, workspace, services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: 42,
                external_path: None,
                repository: None,
                icon_path: None,
            },
        )
        .await;

    let create_collection_output = create_collection_result.unwrap();

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_collections(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the directory was created
    assert!(create_collection_output.abs_path.exists());

    // Verify the db entries were created
    let id = create_collection_output.id;
    let storage_service = services.get::<StorageService<MockAppRuntime>>();
    let item_store = storage_service.storage().item_store();

    // Check order was stored
    let order_key = SEGKEY_COLLECTION.join(&id.to_string()).join("order");
    let order_value = GetItem::get(item_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: usize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 42);

    // Check expanded_items contains the collection id
    let expanded_items_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
    )
    .await
    .unwrap();
    let expanded_items: Vec<CollectionId> = expanded_items_value.deserialize().unwrap();
    assert!(expanded_items.contains(&id));

    cleanup().await;
}

#[tokio::test]
async fn create_collection_with_repo() {
    let (ctx, _workspace_path, workspace, services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let repo = "https://github.com/moss-foundation/sapic.git".to_string();
    let normalized_repo = "github.com/moss-foundation/sapic";
    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: 0,
                external_path: None,
                repository: Some(repo),
                icon_path: None,
            },
        )
        .await;

    let create_collection_output = create_collection_result.unwrap();

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_collections(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the directory was created
    assert!(create_collection_output.abs_path.exists());

    // Verify that the repo is stored in the manifest model
    let id = create_collection_output.id;
    let collection_service = services.get::<CollectionService<MockAppRuntime>>();
    let collection = collection_service.collection(&id).await.unwrap();
    assert_eq!(
        collection.manifest().await.repository,
        Some(normalized_repo.to_string())
    );

    // Verify the db entries were created
    let storage_service = services.get::<StorageService<MockAppRuntime>>();
    let item_store = storage_service.storage().item_store();

    // Check order was stored
    let order_key = SEGKEY_COLLECTION.join(&id.to_string()).join("order");
    let order_value = GetItem::get(item_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: usize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 0);

    // Check expanded_items contains the collection id
    let expanded_items_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
    )
    .await
    .unwrap();
    let expanded_items: Vec<CollectionId> = expanded_items_value.deserialize().unwrap();
    assert!(expanded_items.contains(&id));

    cleanup().await;
}

#[tokio::test]
async fn create_collection_with_icon() {
    let (ctx, workspace_path, workspace, services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let input_icon_path = workspace_path.join("test_icon.png");
    generate_random_icon(&input_icon_path);

    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: 0,
                external_path: None,
                repository: None,
                icon_path: Some(input_icon_path.clone()),
            },
        )
        .await;

    let create_collection_output = create_collection_result.unwrap();

    let id = create_collection_output.id;

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_collections(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the directory was created
    let collection_path = create_collection_output.abs_path;
    assert!(collection_path.exists());

    // Verify that the icon is stored in the assets folder
    let collection = services
        .get::<CollectionService<MockAppRuntime>>()
        .collection(&id)
        .await
        .unwrap();
    assert!(collection.icon_path().is_some());

    // Check order was stored
    let storage_service = services.get::<StorageService<MockAppRuntime>>();
    let item_store = storage_service.storage().item_store();

    let order_key = SEGKEY_COLLECTION.join(&id.to_string()).join("order");
    let order_value = GetItem::get(item_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: usize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 0);

    // Check expanded_items contains the collection id
    let expanded_items_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
    )
    .await
    .unwrap();
    let expanded_items: Vec<CollectionId> = expanded_items_value.deserialize().unwrap();
    assert!(expanded_items.contains(&id));

    cleanup().await;
}

#[tokio::test]
async fn create_multiple_collections_expanded_items() {
    let (ctx, _workspace_path, workspace, services, cleanup) = setup_test_workspace().await;

    // Create first collection
    let collection_name1 = random_collection_name();
    let create_result1 = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name1.clone(),
                order: 0,
                external_path: None,
                repository: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();

    // Create second collection
    let collection_name2 = random_collection_name();
    let create_result2 = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name2.clone(),
                order: 1,
                external_path: None,
                repository: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();

    // Check expanded_items contains both collection ids
    let storage_service = services.get::<StorageService<MockAppRuntime>>();
    let item_store = storage_service.storage().item_store();
    let expanded_items_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
    )
    .await
    .unwrap();
    let expanded_items: Vec<CollectionId> = expanded_items_value.deserialize().unwrap();

    assert_eq!(expanded_items.len(), 2);
    assert!(expanded_items.contains(&create_result1.id));
    assert!(expanded_items.contains(&create_result2.id));

    cleanup().await;
}
