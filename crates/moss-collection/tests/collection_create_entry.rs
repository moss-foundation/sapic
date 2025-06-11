use moss_collection::{
    models::{operations::CreateEntryInput, primitives::WorktreeChange, types::Classification},
    worktree::constants::CONFIG_FILE_NAME_ITEM,
};
use std::{fs::remove_dir_all, path::PathBuf};

use crate::shared::setup_test_collection;

mod shared;

#[tokio::test]
pub async fn test_create_entry_success() {
    let (collection_path, mut collection) = setup_test_collection().await;

    let create_entry_result = collection
        .create_entry(CreateEntryInput {
            destination: PathBuf::from("item"),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await;

    let changes = create_entry_result.unwrap().changes;

    assert_eq!(changes.len(), 1);
    assert!(
        changes
            .iter()
            .any(|item| matches!(item, WorktreeChange::Created { .. }))
    );
    assert!(collection_path.join("item").exists());
    assert!(
        collection_path
            .join("item")
            .join(CONFIG_FILE_NAME_ITEM)
            .exists()
    );

    remove_dir_all(collection_path).unwrap();
}
