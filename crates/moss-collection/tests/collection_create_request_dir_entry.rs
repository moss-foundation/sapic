mod shared;

use std::path::PathBuf;

use moss_collection::models::operations::CreateRequestDirEntryInput;
use moss_collection::models::types::PathChangeKind;
use moss_common::api::{OperationError, OperationResult};

use crate::shared::{random_request_dir_name, set_up_test_collection};

#[tokio::test]
async fn create_request_dir_entry_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_dir_name = random_request_dir_name();

    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: PathBuf::from("requests").join(&request_dir_name),
        })
        .await;

    let changed_paths = create_result.unwrap().changed_paths;

    assert_eq!(changed_paths.len(), 2);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests") && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests").join(&request_dir_name)
            && kind == &PathChangeKind::Created
    }));

    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_request_dir_entry_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_dir_name = random_request_dir_name();

    let _ = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: PathBuf::from("requests").join(&request_dir_name),
        })
        .await;

    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: PathBuf::from("requests").join(&request_dir_name),
        })
        .await;

    assert!(matches!(
        create_result,
        OperationResult::Err(OperationError::AlreadyExists { .. })
    ));

    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_request_dir_entry_nested() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();
    let inner_request_dir_name = random_request_dir_name();
    // requests\group
    // requests\group\inner
    let _ = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: PathBuf::from("requests").join(&request_dir_name),
        })
        .await
        .unwrap();

    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: PathBuf::from("requests")
                .join(&request_dir_name)
                .join(&inner_request_dir_name),
        })
        .await;
    let changed_paths = create_result.unwrap().changed_paths;

    assert_eq!(changed_paths.len(), 1);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join(&request_dir_name)
                .join(&inner_request_dir_name)
            && kind == &PathChangeKind::Created
    }));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
