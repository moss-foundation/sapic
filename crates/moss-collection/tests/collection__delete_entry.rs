pub mod shared;

use moss_collection::{
    dirs,
    models::{
        operations::{CreateDirEntryInput, CreateEntryInput, DeleteEntryInput},
        types::configuration::{
            DirConfigurationModel, HttpDirConfigurationModel, RequestDirConfigurationModel,
        },
    },
};
use moss_common::api::OperationError;
use moss_testutils::random_name::random_string;
use std::path::PathBuf;
use uuid::Uuid;

use crate::shared::create_test_collection;

fn random_entry_name() -> String {
    format!("Test_{}_Entry", random_string(10))
}

fn create_test_dir_configuration() -> DirConfigurationModel {
    DirConfigurationModel::Request(RequestDirConfigurationModel::Http(
        HttpDirConfigurationModel {},
    ))
}

async fn create_test_entry(
    collection: &mut moss_collection::Collection,
    entry_name: &str,
    dir_name: &str,
) -> (Uuid, PathBuf) {
    let entry_path = PathBuf::from(dir_name);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.to_string(),
        order: None,
        configuration: create_test_dir_configuration(),
    });

    let result = collection.create_entry(input).await.unwrap();
    (result.id, entry_path)
}

#[tokio::test]
async fn delete_entry_success() {
    let (collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let (entry_id, entry_path) =
        create_test_entry(&mut collection, &entry_name, dirs::COMPONENTS_DIR).await;

    // Verify entry was created
    let expected_dir = collection_path.join(&entry_path);
    assert!(expected_dir.exists());

    // Delete the entry
    let delete_input = DeleteEntryInput {
        id: entry_id,
        path: entry_path.clone(),
    };

    let result = collection.delete_entry(delete_input).await;
    let _ = result.unwrap();

    // Verify the directory was removed
    assert!(!expected_dir.exists());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn delete_entry_not_found() {
    let (collection_path, mut collection) = create_test_collection().await;

    let non_existent_path = PathBuf::from(dirs::COMPONENTS_DIR).join("non_existent_entry");
    let delete_input = DeleteEntryInput {
        id: Uuid::new_v4(),
        path: non_existent_path,
    };

    let result = collection.delete_entry(delete_input).await;
    assert!(result.is_err());

    if let Err(error) = result {
        match error {
            OperationError::NotFound(_) => {
                // This is expected
            }
            _ => panic!("Expected NotFound error, got {:?}", error),
        }
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn delete_entry_with_subdirectories() {
    let (collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let (entry_id, entry_path) =
        create_test_entry(&mut collection, &entry_name, dirs::COMPONENTS_DIR).await;

    // Create some subdirectories and files inside the entry
    let entry_dir = collection_path.join(&entry_path);
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
    let delete_input = DeleteEntryInput {
        id: entry_id,
        path: entry_path.clone(),
    };

    let result = collection.delete_entry(delete_input).await;
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
    let (collection_path, mut collection) = create_test_collection().await;

    let entry1_name = format!("{}_1", random_entry_name());
    let entry2_name = format!("{}_2", random_entry_name());

    let (entry1_id, entry1_path) =
        create_test_entry(&mut collection, &entry1_name, dirs::COMPONENTS_DIR).await;
    let (entry2_id, entry2_path) =
        create_test_entry(&mut collection, &entry2_name, dirs::SCHEMAS_DIR).await;

    // Verify both entries were created
    let expected_dir1 = collection_path.join(&entry1_path);
    let expected_dir2 = collection_path.join(&entry2_path);
    assert!(expected_dir1.exists());
    assert!(expected_dir2.exists());

    // Delete first entry
    let delete_input1 = DeleteEntryInput {
        id: entry1_id,
        path: entry1_path.clone(),
    };

    let result1 = collection.delete_entry(delete_input1).await;
    let _ = result1.unwrap();

    // Verify first entry was removed, second still exists
    assert!(!expected_dir1.exists());
    assert!(expected_dir2.exists());

    // Delete second entry
    let delete_input2 = DeleteEntryInput {
        id: entry2_id,
        path: entry2_path.clone(),
    };

    let result2 = collection.delete_entry(delete_input2).await;
    let _ = result2.unwrap();

    // Verify both entries are now removed
    assert!(!expected_dir1.exists());
    assert!(!expected_dir2.exists());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn delete_entry_twice() {
    let (collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    let (entry_id, entry_path) =
        create_test_entry(&mut collection, &entry_name, dirs::COMPONENTS_DIR).await;

    // Verify entry was created
    let expected_dir = collection_path.join(&entry_path);
    assert!(expected_dir.exists());

    // Delete the entry first time - should succeed
    let delete_input = DeleteEntryInput {
        id: entry_id,
        path: entry_path.clone(),
    };

    let result1 = collection.delete_entry(delete_input.clone()).await;
    let _ = result1.unwrap();

    // Verify the directory was removed
    assert!(!expected_dir.exists());

    // Try to delete the same entry again - should fail
    let result2 = collection.delete_entry(delete_input).await;
    assert!(result2.is_err());

    if let Err(error) = result2 {
        match error {
            OperationError::NotFound(_) => {
                // This is expected
            }
            _ => panic!("Expected NotFound error, got {:?}", error),
        }
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn delete_entries_from_different_directories() {
    let (collection_path, mut collection) = create_test_collection().await;

    let directories = vec![
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ];

    let mut entries = Vec::new();

    // Create entries in different directories
    for (idx, dir) in directories.iter().enumerate() {
        let entry_name = format!("{}_{}", random_entry_name(), idx);
        let (entry_id, entry_path) = create_test_entry(&mut collection, &entry_name, dir).await;
        entries.push((entry_id, entry_path, dir));
    }

    // Verify all entries were created
    for (_, entry_path, _) in &entries {
        let expected_dir = collection_path.join(entry_path);
        assert!(
            expected_dir.exists(),
            "Entry not created at {:?}",
            entry_path
        );
    }

    // Delete all entries
    for (entry_id, entry_path, _) in &entries {
        let delete_input = DeleteEntryInput {
            id: *entry_id,
            path: entry_path.clone(),
        };

        let result = collection.delete_entry(delete_input).await;
        let _ = result.expect(&format!("Failed to delete entry at {:?}", entry_path));

        // Verify the directory was removed
        let expected_dir = collection_path.join(entry_path);
        assert!(
            !expected_dir.exists(),
            "Entry not deleted at {:?}",
            entry_path
        );
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}
