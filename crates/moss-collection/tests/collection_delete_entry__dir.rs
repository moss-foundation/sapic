use moss_collection::models::operations::{CreateEntryInput, DeleteEntryInput, DeleteEntryOutput};
use moss_collection::models::types::{Classification, PathChangeKind};
use moss_common::api::{OperationError, OperationResult};
use moss_fs::utils::sanitize_path;
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_name;
use std::path::Path;
use std::time::Duration;

use crate::shared::{
    find_id_by_path, random_dir_name, request_folder_name, set_up_test_collection,
};

mod shared;

#[tokio::test]
async fn delete_entry_dir_success() {
    let (collection_path, collection) = set_up_test_collection().await;
    let dir_name = random_dir_name();

    let destination = Path::new("requests").join(&dir_name);

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    let delete_result = collection.delete_entry(DeleteEntryInput { id }).await;

    let DeleteEntryOutput {
        physical_changes,
        virtual_changes,
    } = delete_result.unwrap();

    // Physical
    // requests\\{dir_name}
    // requests\\{dir_name}\\folder.sapic

    // Virtual
    // requests\\{dir_name}

    assert_eq!(physical_changes.len(), 2);
    assert!(
        physical_changes
            .iter()
            .any(|(path, _id, kind)| path.to_path_buf() == destination
                && kind == &PathChangeKind::Removed)
    );
    assert!(
        physical_changes
            .iter()
            .any(
                |(path, _id, kind)| path.to_path_buf() == destination.join("folder.sapic")
                    && kind == &PathChangeKind::Removed
            )
    );

    assert_eq!(virtual_changes.len(), 1);
    assert!(
        virtual_changes
            .iter()
            .any(|(path, _id, _kind)| path.to_path_buf() == destination)
    );

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!collection_path.join(&destination).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_entry_dir_nonexistent_key() {
    let (collection_path, collection) = set_up_test_collection().await;

    let dir_name = random_dir_name();
    let destination = Path::new("requests").join(&dir_name);
    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    // Try deleting the same key twice
    let _ = collection.delete_entry(DeleteEntryInput { id }).await;

    let delete_result = collection.delete_entry(DeleteEntryInput { id }).await;
    assert!(matches!(
        delete_result,
        OperationResult::Err(OperationError::NotFound(..))
    ));

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_entry_dir_with_content() {
    let (collection_path, collection) = set_up_test_collection().await;

    let dir_name = random_dir_name();
    let request_name = random_request_name();

    let dir_destination = Path::new("requests").join(&dir_name);

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: dir_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let dir_id = find_id_by_path(&create_result.virtual_changes, &dir_destination).unwrap();

    // Create a request entry inside the directory
    let _ = collection
        .create_entry(CreateEntryInput {
            destination: dir_destination.join(&request_name),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let delete_result = collection
        .delete_entry(DeleteEntryInput { id: dir_id })
        .await;

    let DeleteEntryOutput {
        physical_changes,
        virtual_changes,
    } = delete_result.unwrap();

    // Physical
    // requests\\{dir_name}
    // requests\\{dir_name}\\folder.sapic
    // requests\\{dir_name}\\{request_name}.request
    // requests\\{dir_name}\\{request_name}.request\\get.sapic

    // Virtual
    // requests\\{dir_name}
    // requests\\{dir_name}\\{request_name}

    assert_eq!(physical_changes.len(), 4);
    assert!(
        physical_changes
            .iter()
            .any(|(path, _id, kind)| path.to_path_buf() == dir_destination
                && kind == &PathChangeKind::Removed)
    );
    assert!(
        physical_changes
            .iter()
            .any(
                |(path, _id, kind)| path.to_path_buf() == dir_destination.join("folder.sapic")
                    && kind == &PathChangeKind::Removed
            )
    );
    assert!(
        physical_changes
            .iter()
            .any(|(path, _id, kind)| path.to_path_buf()
                == dir_destination.join(request_folder_name(&request_name))
                && kind == &PathChangeKind::Removed)
    );
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == dir_destination
                .join(request_folder_name(&request_name))
                .join("get.sapic")
            && kind == &PathChangeKind::Removed
    }));

    assert_eq!(virtual_changes.len(), 2);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == dir_destination && kind == &PathChangeKind::Removed
    }));
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == dir_destination.join(&request_name)
            && kind == &PathChangeKind::Removed
    }));

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!collection_path.join(&dir_destination).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_entry_dir_fs_already_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;

    let dir_name = random_dir_name();
    let destination = Path::new("requests").join(&dir_name);

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    // Delete the directory from the filesystem first
    tokio::fs::remove_dir_all(&collection_path.join(&destination))
        .await
        .unwrap();

    let delete_result = collection.delete_entry(DeleteEntryInput { id }).await;

    let DeleteEntryOutput {
        physical_changes,
        virtual_changes,
    } = delete_result.unwrap();

    // Physical
    // requests\\{dir_name}
    // requests\\{dir_name}\\folder.sapic

    // Virtual
    // requests\\{dir_name}

    assert_eq!(physical_changes.len(), 2);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == destination && kind == &PathChangeKind::Removed
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == destination.join("folder.sapic") && kind == &PathChangeKind::Removed
    }));

    assert_eq!(virtual_changes.len(), 1);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == destination && kind == &PathChangeKind::Removed
    }));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_entry_dir_subfolder() {
    let (collection_path, collection) = set_up_test_collection().await;
    let outer_dir_name = random_dir_name();
    let inner_dir_name = random_dir_name();

    let outer_destination = Path::new("requests").join(&outer_dir_name);
    let inner_destination = outer_destination.join(&inner_dir_name);

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: outer_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let create_inner_result = collection
        .create_entry(CreateEntryInput {
            destination: inner_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let inner_id =
        find_id_by_path(&create_inner_result.virtual_changes, &inner_destination).unwrap();

    let delete_result = collection
        .delete_entry(DeleteEntryInput { id: inner_id })
        .await;

    let DeleteEntryOutput {
        physical_changes,
        virtual_changes,
    } = delete_result.unwrap();

    // Physical
    // requests\\outer\\inner
    // requests\\outer\\inner\\golder.sapic

    // Virtual
    // requests\\outer\\inner

    assert_eq!(physical_changes.len(), 2);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == inner_destination && kind == &PathChangeKind::Removed
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == inner_destination.join("folder.sapic")
            && kind == &PathChangeKind::Removed
    }));

    assert_eq!(virtual_changes.len(), 1);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == inner_destination && kind == &PathChangeKind::Removed
    }));
    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!collection_path.join(&inner_destination).exists());
    assert!(collection_path.join(&outer_destination).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_entry_dir_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;
    let dir_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{s}{}", random_dir_name()))
        .collect::<Vec<_>>();

    for name in dir_name_list {
        let destination = Path::new("requests").join(&name);
        let dir_path = sanitize_path(&destination, None).unwrap();
        let create_result = collection
            .create_entry(CreateEntryInput {
                destination: destination.clone(),
                classification: Classification::Request,
                specification: None,
                protocol: None,
                order: None,
                is_dir: true,
            })
            .await
            .unwrap();

        let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

        let delete_result = collection.delete_entry(DeleteEntryInput { id }).await;
        let DeleteEntryOutput {
            physical_changes,
            virtual_changes,
        } = delete_result.unwrap();

        // Physical
        // requests\\{encoded_dir_name}
        // requests\\{encoded_dir_name}\\folder.sapic

        // Virtual
        // requests\\{dir_name}
        assert_eq!(physical_changes.len(), 2);
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == dir_path && kind == &PathChangeKind::Removed
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == dir_path.join("folder.sapic") && kind == &PathChangeKind::Removed
        }));

        assert_eq!(virtual_changes.len(), 1);
        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == destination && kind == &PathChangeKind::Removed
        }));
    }
    // Wait for spawned deletion tasks to finish
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Check that all dir entries are deleted
    let mut read_dir = collection_path.join("requests").read_dir().unwrap();

    assert!(read_dir.next().is_none());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
