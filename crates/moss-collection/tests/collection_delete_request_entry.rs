mod shared;

use crate::shared::set_up_test_collection;
use moss_collection::models::operations::{
    CreateRequestDirEntryInput, CreateRequestEntryInput, DeleteRequestEntryInput,
};
use moss_collection::models::types::PathChangeKind;
use moss_common::api::{OperationError, OperationResult};
use moss_testutils::random_name::{random_request_dir_name, random_request_name};
use std::path::PathBuf;
use std::time::Duration;

#[tokio::test]
async fn delete_request_entry_success() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let request_folder = PathBuf::from("requests").join(format!("{request_name}.request"));
    // Delete requests/test/{request_name}.request Entry
    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == request_folder)
        .unwrap()
        .1;

    let changed_paths = collection
        .delete_request_entry(DeleteRequestEntryInput { id })
        .await
        .unwrap()
        .changed_paths;

    assert_eq!(changed_paths.len(), 2);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_folder && kind == &PathChangeKind::Removed
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_folder.join("get.sapic") && kind == &PathChangeKind::Removed
    }));

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!collection_path.join(&request_folder).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_entry_nonexistent_key() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let request_folder = PathBuf::from("requests").join(format!("{request_name}.request"));
    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == request_folder)
        .unwrap()
        .1;

    let _ = collection
        .delete_request_entry(DeleteRequestEntryInput { id })
        .await
        .unwrap()
        .changed_paths;

    // Delete the same entry twice
    let result = collection
        .delete_request_entry(DeleteRequestEntryInput { id })
        .await;

    assert!(matches!(
        result,
        OperationResult::Err(OperationError::NotFound { .. })
    ));

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_entry_nested() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join("group").join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let request_folder = PathBuf::from("requests")
        .join("group")
        .join(format!("{request_name}.request"));
    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == request_folder)
        .unwrap()
        .1;

    let changed_paths = collection
        .delete_request_entry(DeleteRequestEntryInput { id })
        .await
        .unwrap()
        .changed_paths;

    assert_eq!(changed_paths.len(), 2);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_folder && kind == &PathChangeKind::Removed
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_folder.join("get.sapic") && kind == &PathChangeKind::Removed
    }));

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!collection_path.join(&request_folder).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_entry_fs_already_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let request_folder = PathBuf::from("requests").join(format!("{request_name}.request"));
    // Delete the entry from the filesystem first
    tokio::fs::remove_dir_all(&collection_path.join(&request_folder))
        .await
        .unwrap();

    // Delete requests/test/{request_name}.request Entry
    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == request_folder)
        .unwrap()
        .1;

    let changed_paths = collection
        .delete_request_entry(DeleteRequestEntryInput { id })
        .await
        .unwrap()
        .changed_paths;

    assert_eq!(changed_paths.len(), 2);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_folder && kind == &PathChangeKind::Removed
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == request_folder.join("get.sapic") && kind == &PathChangeKind::Removed
    }));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_entry_incorrect_entity_type() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();

    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: PathBuf::from("requests").join(&request_dir_name),
        })
        .await;

    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf() == PathBuf::from("requests").join(&request_dir_name)
        })
        .unwrap()
        .1;

    let result = collection
        .delete_request_entry(DeleteRequestEntryInput { id })
        .await;
    assert!(matches!(
        result,
        OperationResult::Err(OperationError::Validation(..))
    ));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
