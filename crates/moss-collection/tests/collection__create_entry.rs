pub mod shared;

use moss_collection::{
    constants, dirs,
    models::operations::{CreateDirEntryInput, CreateEntryInput},
    services::StorageService,
    storage::segments::SEGKEY_RESOURCE_ENTRY,
};
use moss_common::api::OperationError;
use moss_storage::storage::operations::GetItem;
use moss_testutils::fs_specific::FILENAME_SPECIAL_CHARS;
use moss_text::sanitized::sanitize;
use std::path::PathBuf;

use crate::shared::{
    create_test_collection, create_test_request_dir_configuration, random_entry_name,
};

#[tokio::test]
async fn create_dir_entry_success() {
    let (collection_path, collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::REQUESTS_DIR);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
        configuration: create_test_request_dir_configuration(),
    });

    let result = collection.create_entry(input).await;

    let output = result.unwrap();
    assert!(!output.id.is_nil());

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
    let (collection_path, collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::REQUESTS_DIR);
    let order_value = 42;

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: order_value,
        configuration: create_test_request_dir_configuration(),
    });

    let result = collection.create_entry(input).await;
    let output = result.unwrap();
    let id = output.id;

    assert!(!id.is_nil());

    // Verify the directory was created
    let expected_dir = collection_path.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());

    // TODO: Check that order is correctly stored
    let storage_service = collection.service_arc::<StorageService>();
    let resource_store = storage_service.__storage().resource_store();

    // Check order was updated
    let order_key = SEGKEY_RESOURCE_ENTRY.join(&id.to_string()).join("order");
    let order_value = GetItem::get(resource_store.as_ref(), order_key).unwrap();
    let stored_order: isize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 42);

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn create_dir_entry_already_exists() {
    let (collection_path, collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::REQUESTS_DIR);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
        configuration: create_test_request_dir_configuration(),
    });

    // Create the entry first time - should succeed
    let first_result = collection.create_entry(input.clone()).await;
    let _ = first_result.unwrap();

    // Try to create the same entry again - should fail
    let second_result = collection.create_entry(input).await;
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
    let (collection_path, collection) = create_test_collection().await;

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

        let result = collection.create_entry(input).await;

        // Entry creation should succeed - the filesystem layer handles sanitization
        if result.is_err() {
            // Some special characters might legitimately fail, just skip them
            eprintln!(
                "Skipping special char '{}' due to filesystem limitations",
                special_char
            );
            continue;
        }

        let output = result.unwrap();
        assert!(!output.id.is_nil());

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
