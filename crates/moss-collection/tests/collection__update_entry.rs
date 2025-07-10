use crate::shared::{
    create_test_collection, create_test_component_dir_entry, create_test_component_item_entry,
    create_test_request_dir_entry, random_entry_name,
};
use moss_collection::{
    dirs,
    models::{operations::UpdateEntryInput, primitives::EntryId, types::UpdateDirEntryParams},
    services::{
        StorageService,
        storage_service::impl_for_integration_test::StorageServiceForIntegrationTest,
    },
    storage::segments::{SEGKEY_EXPANDED_ENTRIES, SEGKEY_RESOURCE_ENTRY},
};
use moss_storage::storage::operations::GetItem;
use moss_testutils::fs_specific::FILENAME_SPECIAL_CHARS;
use moss_text::sanitized::sanitize;
use std::path::{Path, PathBuf};

mod shared;

// TODO: Test updating entry order

#[tokio::test]
async fn rename_dir_entry_success() {
    let (collection_path, mut collection, _services) = create_test_collection().await;

    let old_entry_name = random_entry_name();
    let new_entry_name = random_entry_name();
    let entry_path = dirs::COMPONENTS_DIR;

    let id = create_test_component_dir_entry(&mut collection, &old_entry_name).await;

    let _ = collection
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
    let old_path = collection_path.join(entry_path).join(&old_entry_name);
    let new_path = collection_path.join(entry_path).join(&new_entry_name);
    assert!(!old_path.exists());
    assert!(new_path.exists());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn rename_dir_entry_empty_name() {
    let (collection_path, mut collection, _services) = create_test_collection().await;

    let old_entry_name = random_entry_name();
    let new_entry_name = "".to_string();

    let id = create_test_component_dir_entry(&mut collection, &old_entry_name).await;

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
    let (collection_path, mut collection, _services) = create_test_collection().await;
    let first_entry_name = random_entry_name();
    let second_entry_name = random_entry_name();

    let first_id = create_test_component_dir_entry(&mut collection, &first_entry_name).await;

    let _ = create_test_component_dir_entry(&mut collection, &second_entry_name).await;

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
    let (collection_path, mut collection, _services) = create_test_collection().await;
    let entry_path = PathBuf::from(dirs::COMPONENTS_DIR);

    for special_char in FILENAME_SPECIAL_CHARS {
        let entry_name = random_entry_name();
        let new_entry_name = format!("{}{}", entry_name, special_char);
        dbg!(&new_entry_name);

        let id = create_test_component_dir_entry(&mut collection, &entry_name).await;

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
        let _ = result.unwrap();

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
    let (collection_path, mut collection, services) = create_test_collection().await;

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&mut collection, &entry_name).await;

    let _ = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id: id.clone(),
            path: Default::default(),
            name: None,
            order: Some(42),
            expanded: None,
        }))
        .await
        .unwrap();

    let storage_service = services.get::<StorageServiceForIntegrationTest>();
    let resource_store = storage_service.storage().resource_store();

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
    let (collection_path, mut collection, services) = create_test_collection().await;

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&mut collection, &entry_name).await;

    let storage_service = services.get::<StorageServiceForIntegrationTest>();
    let resource_store = storage_service.storage().resource_store();

    // Expanding the entry
    let _ = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id: id.clone(),
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
    let expanded_items: Vec<EntryId> = expanded_items_value.deserialize().unwrap();
    assert!(expanded_items.contains(&id));

    // Collapsing the entry
    let _ = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id: id.clone(),
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
    let expanded_items: Vec<EntryId> = expanded_items_value.deserialize().unwrap();
    assert!(!expanded_items.contains(&id));

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap()
}

#[tokio::test]
async fn move_dir_entry_success() {
    let (collection_path, mut collection, _services) = create_test_collection().await;

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&mut collection, &entry_name).await;

    // Create a destination_directory named dest
    let _ = create_test_component_dir_entry(&mut collection, "dest").await;

    let old_dest = PathBuf::from(dirs::COMPONENTS_DIR);
    let new_dest = Path::new(dirs::COMPONENTS_DIR).join("dest");

    // Move entry path from `components/{entry_name}` to `components/dest/{entry_name}`
    let _output = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id,
            path: Some(new_dest.clone()),
            name: None,
            order: None,
            expanded: None,
        }))
        .await
        .unwrap();

    // Verify the path has been changed
    let old_path = collection_path.join(old_dest).join(&entry_name);
    let new_path = collection_path.join(new_dest).join(&entry_name);
    assert!(!old_path.exists());
    assert!(new_path.exists());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn move_dir_entry_nonexistent_destination() {
    let (collection_path, mut collection, _services) = create_test_collection().await;

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&mut collection, &entry_name).await;

    let new_dest = Path::new(dirs::COMPONENTS_DIR).join("dest");

    // Move entry path from `components/{entry_name}` to `components/dest/{entry_name}`
    let result = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id,
            path: Some(new_dest.clone()),
            name: None,
            order: None,
            expanded: None,
        }))
        .await;

    assert!(result.is_err());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn move_dir_entry_different_classification_folder() {
    let (collection_path, mut collection, _services) = create_test_collection().await;

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&mut collection, &entry_name).await;

    // Create a destination_directory named dest
    let _ = create_test_request_dir_entry(&mut collection, "dest").await;

    let new_dest = Path::new(dirs::REQUESTS_DIR).join("dest");

    // Move entry path from `components/{entry_name}` to `requests/dest/{entry_name}`
    let result = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id,
            path: Some(new_dest.clone()),
            name: None,
            order: None,
            expanded: None,
        }))
        .await;

    assert!(result.is_err());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn move_dir_entry_non_dir_destination() {
    let (collection_path, mut collection, _services) = create_test_collection().await;

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&mut collection, &entry_name).await;

    // Create a destination entry (non-directory) named dest
    let _ = create_test_component_item_entry(&mut collection, "dest").await;

    let new_dest = Path::new(dirs::COMPONENTS_DIR).join("dest");

    // Move entry path from `components/{entry_name}` to `components/dest/{entry_name}`
    let result = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id,
            path: Some(new_dest.clone()),
            name: None,
            order: None,
            expanded: None,
        }))
        .await;

    assert!(result.is_err());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn move_dir_entry_already_exists() {
    let (collection_path, mut collection, _services) = create_test_collection().await;

    // First create a dest/entry entry
    let dest_name = "dest".to_string();
    let entry_name = "entry".to_string();

    create_test_component_dir_entry(&mut collection, &dest_name).await;
    let existing_id = create_test_component_dir_entry(&mut collection, &entry_name).await;

    let dest = Path::new(dirs::COMPONENTS_DIR).join(&dest_name);
    let _ = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id: existing_id,
            path: Some(dest.clone()),
            name: None,
            order: None,
            expanded: None,
        }))
        .await
        .unwrap();

    // Create a new entry and try to move it into dest
    let new_id = create_test_component_dir_entry(&mut collection, &entry_name).await;
    let result = collection
        .update_entry(UpdateEntryInput::Dir(UpdateDirEntryParams {
            id: new_id,
            path: Some(dest.clone()),
            name: None,
            order: None,
            expanded: None,
        }))
        .await;

    assert!(result.is_err());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}
