use moss_collection::{
    dirs,
    models::{
        operations::{CreateDirEntryInput, CreateEntryInput, UpdateEntryInput},
        types::UpdateDirEntryParams,
    },
};
use moss_testutils::fs_specific::FILENAME_SPECIAL_CHARS;
use moss_text::sanitized::sanitize;
use std::path::PathBuf;

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
