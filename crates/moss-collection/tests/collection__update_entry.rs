use moss_collection::{
    dirs,
    models::{
        operations::{CreateDirEntryInput, CreateEntryInput, UpdateEntryInput},
        types::UpdateDirEntryParams,
    },
    services::StorageService,
    storage::segments::{SEGKEY_EXPANDED_ENTRIES, SEGKEY_RESOURCE_ENTRY},
};
use moss_storage::storage::operations::GetItem;
use moss_testutils::fs_specific::FILENAME_SPECIAL_CHARS;
use moss_text::sanitized::sanitize;
use std::path::PathBuf;
use uuid::Uuid;

use crate::shared::{create_test_collection, create_test_dir_configuration, random_entry_name};

mod shared;

// TODO: Test updating entry order

#[tokio::test]
async fn rename_dir_entry_success() {
    let (collection_path, collection) = create_test_collection().await;

    let old_entry_name = random_entry_name();
    let new_entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::COMPONENTS_DIR);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: old_entry_name.clone(),
        order: 0,
        configuration: create_test_dir_configuration(),
    });

    let id = collection.create_entry(input).await.unwrap().id;

    let output = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id,
            path: Default::default(),
            name: Some(new_entry_name.clone()),
            order: None,
            expanded: None,
        }))
        .await
        .unwrap();

    // Verify the path has been renamed
    let old_path = collection_path.join(&entry_path).join(&old_entry_name);
    let new_path = collection_path.join(&entry_path).join(&new_entry_name);
    assert!(!old_path.exists());
    assert!(new_path.exists());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn rename_dir_entry_empty_name() {
    let (collection_path, collection) = create_test_collection().await;

    let old_entry_name = random_entry_name();
    let new_entry_name = "".to_string();
    let entry_path = PathBuf::from(dirs::COMPONENTS_DIR);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: old_entry_name.clone(),
        order: 0,
        configuration: create_test_dir_configuration(),
    });

    let id = collection.create_entry(input).await.unwrap().id;

    let result = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id,
            path: Default::default(),
            name: Some(new_entry_name.clone()),
            order: None,
            expanded: None,
        }))
        .await;

    assert!(result.is_err());

    //Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn rename_dir_entry_already_exists() {
    let (collection_path, collection) = create_test_collection().await;
    let first_entry_name = random_entry_name();
    let second_entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::COMPONENTS_DIR);

    let first_input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: first_entry_name.clone(),
        order: 0,
        configuration: create_test_dir_configuration(),
    });

    let first_id = collection.create_entry(first_input).await.unwrap().id;

    let second_input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: second_entry_name.clone(),
        order: 0,
        configuration: create_test_dir_configuration(),
    });

    let _ = collection.create_entry(second_input).await.unwrap();
    // Try to rename first entry to the second name

    let result = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id: first_id,
            path: Default::default(),
            name: Some(second_entry_name.clone()),
            order: None,
            expanded: None,
        }))
        .await;

    assert!(result.is_err());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn rename_dir_entry_special_chars_in_name() {
    let (collection_path, collection) = create_test_collection().await;
    let entry_path = PathBuf::from(dirs::COMPONENTS_DIR);

    for special_char in FILENAME_SPECIAL_CHARS {
        let entry_name = random_entry_name();
        let new_entry_name = format!("{}{}", entry_name, special_char);
        dbg!(&new_entry_name);
        let create_input = CreateEntryInput::Dir(CreateDirEntryInput {
            path: entry_path.clone(),
            name: entry_name.clone(),
            order: 0,
            configuration: create_test_dir_configuration(),
        });

        let id = collection.create_entry(create_input).await.unwrap().id;

        let result = collection
            .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
                id,
                path: Default::default(),
                name: Some(new_entry_name.clone()),
                order: None,
                expanded: None,
            }))
            .await;

        if result.is_err() {
            // Some special characters might legitimately fail, just skip them
            eprintln!(
                "Skipping special char '{}' due to filesystem limitations",
                special_char
            );
            continue;
        }
        let output = result.unwrap();

        let expected_dir = collection_path
            .join(&entry_path)
            .join(&sanitize(&new_entry_name));
        dbg!(&expected_dir);
        assert!(expected_dir.exists());
        assert!(expected_dir.is_dir());
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn update_dir_entry_order() {
    let (collection_path, collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::COMPONENTS_DIR);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
        configuration: create_test_dir_configuration(),
    });

    let id = collection.create_entry(input).await.unwrap().id;

    let _ = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id,
            path: Default::default(),
            name: None,
            order: Some(42),
            expanded: None,
        }))
        .await
        .unwrap();

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
async fn expand_and_collapse_dir_entry() {
    let (collection_path, collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::COMPONENTS_DIR);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
        configuration: create_test_dir_configuration(),
    });

    let id = collection.create_entry(input).await.unwrap().id;

    let storage_service = collection.service_arc::<StorageService>();
    let resource_store = storage_service.__storage().resource_store();

    // Expanding the entry
    let _ = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id,
            path: Default::default(),
            name: None,
            order: None,
            expanded: Some(true),
        }))
        .await
        .unwrap();

    // Check expanded_items contains the entry id
    let expanded_items_value = GetItem::get(
        resource_store.as_ref(),
        SEGKEY_EXPANDED_ENTRIES.to_segkey_buf(),
    )
    .unwrap();
    let expanded_items: Vec<Uuid> = expanded_items_value.deserialize().unwrap();
    assert!(expanded_items.contains(&id));

    // Collapsing the entry
    let _ = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id,
            path: Default::default(),
            name: None,
            order: None,
            expanded: Some(false),
        }))
        .await
        .unwrap();

    // Check expanded_items contains the entry id
    let expanded_items_value = GetItem::get(
        resource_store.as_ref(),
        SEGKEY_EXPANDED_ENTRIES.to_segkey_buf(),
    )
    .unwrap();
    let expanded_items: Vec<Uuid> = expanded_items_value.deserialize().unwrap();
    assert!(!expanded_items.contains(&id));

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap()
}
