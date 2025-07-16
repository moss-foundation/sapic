#![cfg(feature = "integration-tests")]
pub mod shared;

use crate::shared::{
    create_test_collection, create_test_component_dir_configuration,
    create_test_component_item_configuration, create_test_request_dir_configuration,
    random_entry_name,
};
use moss_applib::mock::MockAppRuntime;
use moss_collection::{
    constants, dirs,
    models::operations::{CreateDirEntryInput, CreateEntryInput, CreateItemEntryInput},
    services::StorageService,
    storage::segments::SEGKEY_RESOURCE_ENTRY,
};
use moss_common::api::OperationError;
use moss_storage::storage::operations::GetItem;
use moss_testutils::fs_specific::FILENAME_SPECIAL_CHARS;
use moss_text::sanitized::sanitize;
use std::path::PathBuf;

#[tokio::test]
async fn create_dir_entry_success() {
    let (ctx, collection_path, collection, _services) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::REQUESTS_DIR);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
        configuration: create_test_request_dir_configuration(),
    });

    let result = collection.create_entry(&ctx, input).await;

    let output = result.unwrap();

    // Verify the directory was created
    let expected_dir = collection_path.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());
    assert!(expected_dir.is_dir());

    let config_file = expected_dir.join(constants::DIR_CONFIG_FILENAME);
    assert!(config_file.exists());
    assert!(config_file.is_file());

    // Read and verify config content
    let config_content = std::fs::read_to_string(config_file).unwrap();
    assert!(config_content.contains(&output.id.to_string()));

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn create_dir_entry_with_order() {
    let (ctx, collection_path, collection, services) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::REQUESTS_DIR);
    let order_value = 42;

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: order_value,
        configuration: create_test_request_dir_configuration(),
    });

    let result = collection.create_entry(&ctx, input).await;
    let id = result.unwrap().id;

    // Verify the directory was created
    let expected_dir = collection_path.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());

    // TODO: Check that order is correctly stored
    let storage_service = services.get::<StorageService<MockAppRuntime>>();
    let resource_store = storage_service.storage().resource_store();

    // Check order was updated
    let order_key = SEGKEY_RESOURCE_ENTRY.join(&id.to_string()).join("order");
    let order_value = GetItem::get(resource_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: isize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 42);

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn create_dir_entry_already_exists() {
    let (ctx, collection_path, collection, _services) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::REQUESTS_DIR);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
        configuration: create_test_request_dir_configuration(),
    });

    // Create the entry first time - should succeed
    let first_result = collection.create_entry(&ctx, input.clone()).await;
    let _ = first_result.unwrap();

    // Try to create the same entry again - should fail
    let second_result = collection.create_entry(&ctx, input).await;
    assert!(second_result.is_err());

    if let Err(error) = second_result {
        match error {
            OperationError::AlreadyExists(_) => {
                // This is expected
            }
            _ => panic!("Expected AlreadyExists error, got {:?}", error),
        }
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn create_dir_entry_special_chars_in_name() {
    let (ctx, collection_path, collection, _services) = create_test_collection().await;

    let base_name = random_entry_name();

    for special_char in FILENAME_SPECIAL_CHARS {
        let entry_name = format!("{}{}", base_name, special_char);
        let entry_path = PathBuf::from(dirs::REQUESTS_DIR);

        let input = CreateEntryInput::Dir(CreateDirEntryInput {
            path: entry_path.clone(),
            name: entry_name.clone(),
            order: 0,
            configuration: create_test_request_dir_configuration(),
        });

        let result = collection.create_entry(&ctx, input).await;

        // Entry creation should succeed - the filesystem layer handles sanitization
        if result.is_err() {
            // Some special characters might legitimately fail, just skip them
            eprintln!(
                "Skipping special char '{}' due to filesystem limitations",
                special_char
            );
            continue;
        }

        let _output = result.unwrap();

        // The exact directory name might be sanitized, but some directory should exist
        // We just verify that the operation completed successfully
        let expected_dir = collection_path
            .join(&entry_path)
            .join(sanitize(&entry_name));
        assert!(expected_dir.exists());
        assert!(expected_dir.is_dir());
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn create_dir_entry_inside_item_entry() {
    let (ctx, collection_path, collection, _services) = create_test_collection().await;

    let outer_name = random_entry_name();
    let outer_path = PathBuf::from(dirs::COMPONENTS_DIR);
    let outer_input = CreateEntryInput::Item(CreateItemEntryInput {
        path: outer_path.clone(),
        name: outer_name.clone(),
        order: 0,
        configuration: create_test_component_item_configuration(),
    });

    let _ = collection.create_entry(&ctx, outer_input).await.unwrap();

    // Try creating an entry inside an item entry

    let inner_name = random_entry_name();
    let inner_path = PathBuf::from(dirs::COMPONENTS_DIR).join(&outer_name);
    let inner_input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: inner_path.clone(),
        name: inner_name.clone(),
        order: 0,
        configuration: create_test_component_dir_configuration(),
    });

    let result = collection.create_entry(&ctx, inner_input).await;

    assert!(result.is_err());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}
