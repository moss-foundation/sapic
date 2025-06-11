use moss_collection::models::{
    operations::{CreateEntryInput, DeleteEntryInput},
    primitives::WorktreeChange,
    types::Classification,
};
use std::{fs::remove_dir_all, path::PathBuf};

use crate::shared::setup_test_collection;

mod shared;

#[tokio::test]
async fn test_delete_entry_success() {
    let (collection_path, mut collection) = setup_test_collection().await;

    let destination = PathBuf::from("item");
    let create_entry_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await;

    let changes = create_entry_result.unwrap().changes;

    let id = changes[0].id();

    let delete_entry_result = collection.delete_entry(DeleteEntryInput { id }).await;

    assert!(delete_entry_result.is_ok());

    let changes = delete_entry_result.unwrap().changes;
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes[0],
        WorktreeChange::Deleted {
            id,
            path: destination.as_path().into()
        }
    );

    remove_dir_all(collection_path).unwrap();
}
