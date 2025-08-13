#![cfg(feature = "integration-tests")]

use moss_collection::{
    constants, dirs,
    models::{
        operations::{BatchCreateEntryInput, BatchCreateEntryKind},
        types::{CreateDirEntryParams, CreateItemEntryParams},
    },
};
use std::path::PathBuf;

use crate::shared::{create_test_collection, random_entry_name};

pub mod shared;

#[tokio::test]
async fn batch_create_entry_success() {
    let (ctx, collection_path, collection) = create_test_collection().await;

    let class_path = PathBuf::from(dirs::COMPONENTS_DIR);

    // components/{outer_name}
    // components/{outer_name}/{inner_name}

    let outer_name = random_entry_name();
    let inner_name = random_entry_name();
    let outer_input = BatchCreateEntryKind::Dir(CreateDirEntryParams {
        path: class_path.clone(),
        name: outer_name.clone(),
        order: 0,
        headers: vec![],
    });
    let inner_input = BatchCreateEntryKind::Item(CreateItemEntryParams {
        path: class_path.join(&outer_name),
        name: inner_name.clone(),
        order: 0,
        protocol: None,
        query_params: vec![],
        path_params: vec![],
        headers: vec![],
    });
    let input = BatchCreateEntryInput {
        // Make sure that the order is correctly sorted
        entries: vec![inner_input, outer_input],
    };

    let output = collection.batch_create_entry(&ctx, input).await.unwrap();
    assert_eq!(output.ids.len(), 2);

    // Verify the directories were created
    let outer_dir = collection_path.join(&class_path).join(&outer_name);
    assert!(outer_dir.exists());
    assert!(outer_dir.is_dir());
    let outer_config = outer_dir.join(constants::DIR_CONFIG_FILENAME);
    assert!(outer_config.exists());
    assert!(outer_config.is_file());

    let inner_dir = outer_dir.join(&inner_name);
    assert!(inner_dir.exists());
    assert!(inner_dir.is_dir());
    let inner_config = inner_dir.join(constants::ITEM_CONFIG_FILENAME);
    assert!(inner_config.exists());
    assert!(inner_config.is_file());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn batch_create_entry_missing_parent() {
    let (ctx, collection_path, collection) = create_test_collection().await;

    let class_path = PathBuf::from(dirs::COMPONENTS_DIR);
    let inner_name = random_entry_name();

    // Try creating components/parent/{inner_name}
    let inner_input = BatchCreateEntryKind::Item(CreateItemEntryParams {
        path: class_path.join("parent"),
        name: inner_name.clone(),
        order: 0,
        protocol: None,
        query_params: vec![],
        path_params: vec![],
        headers: vec![],
    });
    let input = BatchCreateEntryInput {
        entries: vec![inner_input],
    };

    let result = collection.batch_create_entry(&ctx, input).await;
    assert!(result.is_err());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn batch_create_entry_empty_input() {
    let (ctx, collection_path, collection) = create_test_collection().await;

    let input = BatchCreateEntryInput { entries: vec![] };
    let output = collection.batch_create_entry(&ctx, input).await.unwrap();

    assert_eq!(output.ids.len(), 0);

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}
