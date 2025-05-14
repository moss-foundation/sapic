mod shared;

use crate::shared::set_up_test_collection;
use moss_collection::models::operations::{
    CreateRequestDirEntryInput, CreateRequestEntryInput, DeleteRequestDirEntryInput,
};
use moss_collection::models::types::PathChangeKind;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::utils::encode_path;
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::{random_request_dir_name, random_request_name};
use std::path::PathBuf;
use std::time::Duration;

#[tokio::test]
async fn delete_request_dir_entry_success() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();

    let destination = PathBuf::from("requests").join(request_dir_name);

    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: destination.clone(),
        })
        .await
        .unwrap();

    let id = create_result
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == destination)
        .unwrap()
        .1;

    let changed_paths = collection
        .delete_request_dir_entry(DeleteRequestDirEntryInput { id: id.clone() })
        .await
        .unwrap()
        .changed_paths;
    assert_eq!(changed_paths.len(), 1);
    assert!(
        changed_paths
            .iter()
            .any(|(path, i, kind)| path.to_path_buf() == destination
                && i == &id
                && kind == &PathChangeKind::Removed)
    );

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!collection_path.join(&destination).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_dir_entry_nonexistent_key() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_dir_name = random_request_dir_name();
    let destination = PathBuf::from("requests").join(request_dir_name);
    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: destination.clone(),
        })
        .await;

    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == destination)
        .unwrap()
        .1;

    let _ = collection
        .delete_request_dir_entry(DeleteRequestDirEntryInput { id: id.clone() })
        .await
        .unwrap();

    let delete_result = collection
        .delete_request_dir_entry(DeleteRequestDirEntryInput { id: id.clone() })
        .await;

    assert!(matches!(
        delete_result,
        OperationResult::Err(OperationError::NotFound { .. })
    ));
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_dir_entry_with_content() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_dir_name = random_request_dir_name();
    let request_name = random_request_name();

    let dir_destination = PathBuf::from("requests").join(&request_dir_name);
    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: dir_destination.clone(),
        })
        .await;

    let dir_id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == dir_destination)
        .unwrap()
        .1;

    let _ = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: dir_destination.join(&request_name),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let delete_result = collection
        .delete_request_dir_entry(DeleteRequestDirEntryInput { id: dir_id.clone() })
        .await;

    // requests/{group}
    // requests/{group}/{request}
    // requests/{group}/{request}/get.sapic
    let changed_paths = delete_result.unwrap().changed_paths;

    assert_eq!(changed_paths.len(), 3);
    assert!(
        changed_paths
            .iter()
            .any(|(path, _id, kind)| path.to_path_buf() == dir_destination
                && kind == &PathChangeKind::Removed)
    );
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == dir_destination.join(format!("{}.request", &request_name))
            && kind == &PathChangeKind::Removed
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == dir_destination
                .join(format!("{}.request", &request_name))
                .join("get.sapic")
            && kind == &PathChangeKind::Removed
    }));

    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(!collection_path.join(&dir_destination).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_dir_entry_fs_already_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_dir_name = random_request_dir_name();
    let dir_destination = PathBuf::from("requests").join(request_dir_name);

    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: dir_destination.clone(),
        })
        .await;

    let dir_id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == dir_destination)
        .unwrap()
        .1;

    // Delete the request dir from the fs first
    tokio::fs::remove_dir_all(collection_path.join(&dir_destination))
        .await
        .unwrap();

    let delete_result = collection
        .delete_request_dir_entry(DeleteRequestDirEntryInput { id: dir_id.clone() })
        .await;

    let changed_paths = delete_result.unwrap().changed_paths;

    assert_eq!(changed_paths.len(), 1);
    assert!(
        changed_paths
            .iter()
            .any(|(path, id, kind)| path.to_path_buf() == dir_destination
                && id == &dir_id
                && kind == &PathChangeKind::Removed)
    );

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_dir_entry_subfolder() {
    let (collection_path, collection) = set_up_test_collection().await;
    let outer_group_name = random_request_dir_name();
    let inner_group_name = random_request_dir_name();
    let outer_destination = PathBuf::from("requests").join(outer_group_name);
    let _ = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: outer_destination.clone(),
        })
        .await
        .unwrap();

    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: outer_destination.join(&inner_group_name),
        })
        .await;

    let inner_group_id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == outer_destination.join(&inner_group_name))
        .unwrap()
        .1;

    let delete_result = collection
        .delete_request_dir_entry(DeleteRequestDirEntryInput {
            id: inner_group_id.clone(),
        })
        .await;

    let changed_paths = delete_result.unwrap().changed_paths;

    assert_eq!(changed_paths.len(), 1);
    assert!(changed_paths.iter().any(|(path, id, kind)| {
        path.to_path_buf() == outer_destination.join(&inner_group_name)
            && id == &inner_group_id
            && kind == &PathChangeKind::Removed
    }));
    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(
        !collection_path
            .join(&outer_destination)
            .join(&inner_group_name)
            .exists()
    );
    assert!(collection_path.join(&outer_destination).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_dir_entry_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| (format!("{s}{}", random_request_dir_name())))
        .collect::<Vec<_>>();

    for name in request_dir_name_list {
        let destination = PathBuf::from("requests").join(&name);
        let create_result = collection
            .create_request_dir_entry(CreateRequestDirEntryInput {
                destination: destination.clone(),
            })
            .await;

        let dir_id = create_result
            .unwrap()
            .changed_paths
            .into_iter()
            .find(|(path, _id, _kind)| {
                path.to_path_buf() == encode_path(&destination, None).unwrap()
            })
            .unwrap()
            .1;

        let delete_result = collection
            .delete_request_dir_entry(DeleteRequestDirEntryInput { id: dir_id.clone() })
            .await;

        let changed_paths = delete_result.unwrap().changed_paths;
        assert_eq!(changed_paths.len(), 1);
        assert!(changed_paths.iter().any(|(path, id, kind)| {
            path.to_path_buf() == encode_path(&destination, None).unwrap()
                && id == &dir_id
                && kind == &PathChangeKind::Removed
        }));
    }
    // Wait for spawned deletion task to finish
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Check that all request groups are deleted
    let mut read_dir = collection_path.join("requests").read_dir().unwrap();

    assert!(read_dir.next().is_none());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn delete_request_dir_entry_incorrect_entity_type() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let entry_id = create_result
        .unwrap()
        .changed_paths
        .iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf()
                == PathBuf::from("requests").join(format!("{}.request", request_name))
        })
        .unwrap()
        .1;

    let delete_result = collection
        .delete_request_dir_entry(DeleteRequestDirEntryInput {
            id: entry_id.clone(),
        })
        .await;

    assert!(matches!(
        delete_result,
        OperationResult::Err(OperationError::Validation(..))
    ));
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
