#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_collection::{
    dirs,
    errors::ErrorNotFound,
    models::{operations::DeleteEntryInput, primitives::EntryId},
};
use std::path::PathBuf;

use crate::shared::{
    create_test_collection, create_test_component_dir_entry, create_test_endpoint_dir_entry,
    create_test_request_dir_entry, create_test_schema_dir_entry, random_entry_name,
};

#[tokio::test]
async fn delete_entry_success() {
    let (ctx, collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::REQUESTS_DIR);
    let entry_id = create_test_request_dir_entry(&ctx, &mut collection, &entry_name).await;

    // Verify entry was created
    let expected_dir = collection_path.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());

    // Delete the entry
    let delete_input = DeleteEntryInput { id: entry_id };

    let result = collection.delete_entry(&ctx, delete_input).await;
    let _ = result.unwrap();

    // Verify the directory was removed
    assert!(!expected_dir.exists());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn delete_entry_not_found() {
    let (ctx, collection_path, collection) = create_test_collection().await;

    let delete_input = DeleteEntryInput { id: EntryId::new() };

    let result = collection.delete_entry(&ctx, delete_input).await;
    assert!(result.is_err());

    if let Err(error) = result {
        assert!(error.is::<ErrorNotFound>());
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn delete_entry_with_subdirectories() {
    let (ctx, collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::REQUESTS_DIR);
    let entry_id = create_test_request_dir_entry(&ctx, &mut collection, &entry_name).await;

    // Create some subdirectories and files inside the entry
    let entry_dir = collection_path.join(&entry_path).join(&entry_name);
    let sub_dir = entry_dir.join("subdir");
    let sub_sub_dir = sub_dir.join("subsubdir");

    std::fs::create_dir_all(&sub_sub_dir).unwrap();
    std::fs::write(sub_dir.join("test_file.txt"), "test content").unwrap();
    std::fs::write(sub_sub_dir.join("nested_file.md"), "nested content").unwrap();

    // Verify structure was created
    assert!(entry_dir.exists());
    assert!(sub_dir.exists());
    assert!(sub_sub_dir.exists());

    // Delete the entry
    let delete_input = DeleteEntryInput { id: entry_id };

    let result = collection.delete_entry(&ctx, delete_input).await;
    let _ = result.unwrap();

    // Verify the entire directory tree was removed
    assert!(!entry_dir.exists());
    assert!(!sub_dir.exists());
    assert!(!sub_sub_dir.exists());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn delete_multiple_entries() {
    let (ctx, collection_path, mut collection) = create_test_collection().await;

    let entry1_name = format!("{}_1", random_entry_name());
    let entry2_name = format!("{}_2", random_entry_name());

    let entry1_path = PathBuf::from(dirs::REQUESTS_DIR);
    let entry1_id = create_test_request_dir_entry(&ctx, &mut collection, &entry1_name).await;

    let entry2_path = PathBuf::from(dirs::ENDPOINTS_DIR);
    let entry2_id = create_test_endpoint_dir_entry(&ctx, &mut collection, &entry2_name).await;

    // Verify both entries were created
    let expected_dir1 = collection_path.join(&entry1_path).join(&entry1_name);
    let expected_dir2 = collection_path.join(&entry2_path).join(&entry2_name);
    assert!(expected_dir1.exists());
    assert!(expected_dir2.exists());

    // Delete first entry
    let delete_input1 = DeleteEntryInput { id: entry1_id };

    let result1 = collection.delete_entry(&ctx, delete_input1).await;
    let _ = result1.unwrap();

    // Verify first entry was removed, second still exists
    assert!(!expected_dir1.exists());
    assert!(expected_dir2.exists());

    // Delete second entry
    let delete_input2 = DeleteEntryInput { id: entry2_id };

    let result2 = collection.delete_entry(&ctx, delete_input2).await;
    let _ = result2.unwrap();

    // Verify both entries are now removed
    assert!(!expected_dir1.exists());
    assert!(!expected_dir2.exists());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn delete_entry_twice() {
    let (ctx, collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(dirs::REQUESTS_DIR);
    let entry_id = create_test_request_dir_entry(&ctx, &mut collection, &entry_name).await;

    // Verify entry was created
    let expected_dir = collection_path.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());

    // Delete the entry first time - should succeed
    let delete_input = DeleteEntryInput { id: entry_id };

    let result1 = collection.delete_entry(&ctx, delete_input.clone()).await;
    let _ = result1.unwrap();

    // Verify the directory was removed
    assert!(!expected_dir.exists());

    // Try to delete the same entry again - should fail
    let result2 = collection.delete_entry(&ctx, delete_input).await;
    assert!(result2.is_err());

    if let Err(error) = result2 {
        assert!(error.is::<ErrorNotFound>());
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn delete_entries_from_different_directories() {
    let (ctx, collection_path, mut collection) = create_test_collection().await;

    let mut entries = Vec::new();

    // We have to manually do this now, since we will validate path against configuration
    let request_id = create_test_request_dir_entry(&ctx, &mut collection, "entry").await;
    entries.push(request_id);

    let endpoint_id = create_test_endpoint_dir_entry(&ctx, &mut collection, "entry").await;
    entries.push(endpoint_id);

    let component_id = create_test_component_dir_entry(&ctx, &mut collection, "entry").await;
    entries.push(component_id);

    let schema_id = create_test_schema_dir_entry(&ctx, &mut collection, "entry").await;
    entries.push(schema_id);

    // Create entries in different directories
    for entry_id in entries {
        let _ = collection
            .delete_entry(&ctx, DeleteEntryInput { id: entry_id })
            .await
            .unwrap();
    }

    // Verify that all the dir entries are removed
    for dir in [
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ] {
        let expected_dir = collection_path.join(dir).join("entry");
        assert!(
            !expected_dir.exists(),
            "Entry not deleted at {:?}",
            expected_dir
        );
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}
