use moss_collection::models::{
    operations::{CreateEntryInput, DeleteEntryInput, DeleteEntryOutput},
    types::{Classification, PathChangeKind},
};
use moss_common::api::OperationError;
use moss_testutils::{fs_specific::FOLDERNAME_SPECIAL_CHARS, random_name::random_request_name};
use std::{path::Path, time::Duration};

use crate::shared::{create_test_collection, find_id_by_path, request_folder_name};

mod shared;

#[tokio::test]
async fn delete_entry_request_success() {
    let (collection_path, mut collection) = create_test_collection().await;
    let request_name = random_request_name();
    let destination = Path::new("requests").join(&request_name);

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let request_path = Path::new("requests").join(request_folder_name(&request_name));

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    let delete_result = collection.delete_entry(DeleteEntryInput { id }).await;

    let DeleteEntryOutput {
        physical_changes,
        virtual_changes,
    } = delete_result.unwrap();

    // Physical
    // requests\\{request_name}.request
    // requests\\{request_name}.request\\get.sapic

    // Virtual
    // requests\\{request_name}
    assert_eq!(physical_changes.len(), 2);
    assert!(
        physical_changes
            .iter()
            .any(|(path, _id, _kind)| path.to_path_buf() == request_path)
    );
    assert!(
        physical_changes
            .iter()
            .any(|(path, _id, _kind)| path.to_path_buf() == request_path.join("get.sapic"))
    );

    assert_eq!(virtual_changes.len(), 1);
    assert!(
        virtual_changes
            .iter()
            .any(|(path, _id, _kind)| { path.to_path_buf() == destination })
    );

    // Wait for spawned deletion tasks to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!request_path.exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_entry_request_nonexistent_key() {
    let (collection_path, mut collection) = create_test_collection().await;
    let request_name = random_request_name();

    let destination = Path::new("requests").join(&request_name);
    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    // Try deleting the same key twice
    let _ = collection
        .delete_entry(DeleteEntryInput { id })
        .await
        .unwrap();

    let delete_result = collection.delete_entry(DeleteEntryInput { id }).await;

    assert!(matches!(delete_result, Err(OperationError::NotFound(..))));

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_entry_request_nested() {
    let (collection_path, mut collection) = create_test_collection().await;
    let request_name = random_request_name();

    let destination = Path::new("requests").join("group").join(&request_name);
    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    // Find requests\\group\\{request_name}
    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    let delete_result = collection.delete_entry(DeleteEntryInput { id }).await;

    let group_path = Path::new("requests").join("group");
    let request_path = group_path.join(&request_folder_name(&request_name));
    // Physical
    // requests\\group\\{request_name}.request
    // requests\\group\\{request_name}.request\\get.sapic

    // Virtual
    // requests\\group\\{request_name}

    let DeleteEntryOutput {
        physical_changes,
        virtual_changes,
    } = delete_result.unwrap();

    assert_eq!(physical_changes.len(), 2);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_path && kind == &PathChangeKind::Removed
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_path.join("get.sapic") && kind == &PathChangeKind::Removed
    }));

    assert_eq!(virtual_changes.len(), 1);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == destination && kind == &PathChangeKind::Removed
    }));

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!collection_path.join(&request_path).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_entry_request_fs_already_deleted() {
    let (collection_path, mut collection) = create_test_collection().await;

    let request_name = random_request_name();
    let destination = Path::new("requests").join(&request_name);
    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    let request_path = Path::new("requests").join(request_folder_name(&request_name));
    // Delete the entry from the filesystem first
    tokio::fs::remove_dir_all(&collection_path.join(&request_path))
        .await
        .unwrap();

    // Delete the entry
    let delete_result = collection.delete_entry(DeleteEntryInput { id }).await;

    let DeleteEntryOutput {
        physical_changes,
        virtual_changes,
    } = delete_result.unwrap();

    // Physical
    // requests\\{request_name}.request
    // requests\\{request_name}.request\\get.sapic

    // Virtual
    // requests\\{request_name}

    assert_eq!(physical_changes.len(), 2);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_path && kind == &PathChangeKind::Removed
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_path.join("get.sapic") && kind == &PathChangeKind::Removed
    }));

    assert_eq!(virtual_changes.len(), 1);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == destination && kind == &PathChangeKind::Removed
    }));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_entry_request_special_chars() {
    let (collection_path, mut collection) = create_test_collection().await;
    let request_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{s}{}", random_request_name()))
        .collect::<Vec<_>>();

    for name in request_name_list {
        let destination = Path::new("requests").join(&name);
        let request_path = Path::new("requests").join(&request_folder_name(&name));
        let create_result = collection
            .create_entry(CreateEntryInput {
                destination: destination.clone(),
                classification: Classification::Request,
                specification: None,
                protocol: None,
                order: None,
                is_dir: false,
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
        // requests\\{encoded_request_name}.request
        // requests\\{encoded_request_name}.request\\get.sapic

        // Virtual
        // requests\\{request_name}
        assert_eq!(physical_changes.len(), 2);
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == request_path && kind == &PathChangeKind::Removed
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == request_path.join("get.sapic") && kind == &PathChangeKind::Removed
        }));

        assert_eq!(virtual_changes.len(), 1);
        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == destination && kind == &PathChangeKind::Removed
        }));
    }

    // Wait for spawned deletion tasks to finish
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Check taht all request entries are deleted
    let mut read_dir = collection_path.join("requests").read_dir().unwrap();

    assert!(read_dir.next().is_none());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
