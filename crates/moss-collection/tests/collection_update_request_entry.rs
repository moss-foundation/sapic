mod shared;

use moss_collection::models::operations::{
    CreateRequestDirEntryInput, CreateRequestEntryInput, DeleteRequestEntryInput,
    UpdateRequestEntryInput,
};
use moss_collection::models::types::PathChangeKind;
use moss_common::api::{OperationError, OperationResult};
use moss_testutils::random_name::random_request_name;
use std::path::PathBuf;
use std::time::Duration;

use crate::shared::{random_request_dir_name, request_folder_name, set_up_test_collection};

#[tokio::test]
async fn update_request_entry_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let request_new_name = format!("{request_name}_new");

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf() == PathBuf::from("requests").join(request_folder_name(&request_name))
        })
        .unwrap()
        .1;

    let update_result = collection
        .update_request_entry(UpdateRequestEntryInput {
            id: id.clone(),
            name: Some(request_new_name.clone()),
        })
        .await;

    let changed_paths = update_result.unwrap().changed_paths;

    let old_request_path = PathBuf::from("requests").join(request_folder_name(&request_name));
    let new_request_path = PathBuf::from("requests").join(request_folder_name(&request_new_name));
    assert_eq!(changed_paths.len(), 2);
    assert!(changed_paths.into_iter().any(|(path, _id, kind)| {
        path.to_path_buf() == new_request_path && kind == &PathChangeKind::Updated
    }));
    assert!(changed_paths.into_iter().any(|(path, _id, kind)| {
        path.to_path_buf() == new_request_path.join("get.sapic") && kind == &PathChangeKind::Updated
    }));

    assert!(!collection_path.join(&old_request_path).exists());
    assert!(collection_path.join(&new_request_path).exists());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_entry_no_change() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf() == PathBuf::from("requests").join(request_folder_name(&request_name))
        })
        .unwrap()
        .1;

    let update_result = collection
        .update_request_entry(UpdateRequestEntryInput { id, name: None })
        .await;

    let changed_paths = update_result.unwrap().changed_paths;
    assert!(changed_paths.is_empty());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_entry_same_name() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();
    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf() == PathBuf::from("requests").join(request_folder_name(&request_name))
        })
        .unwrap()
        .1;

    let update_result = collection
        .update_request_entry(UpdateRequestEntryInput {
            id,
            name: Some(request_name.clone()),
        })
        .await;

    let changed_paths = update_result.unwrap().changed_paths;
    assert!(changed_paths.is_empty());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_entry_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;
    let first_destination = PathBuf::from("requests").join("first");
    let second_destination = PathBuf::from("requests").join("second");

    let create_request_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: first_destination.clone(),
            url: None,
            payload: None,
        })
        .await;

    let first_id = create_request_result
        .unwrap()
        .changed_paths
        .iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf() == PathBuf::from("requests").join("first.request")
        })
        .unwrap()
        .1;

    let _ = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: second_destination.clone(),
            url: None,
            payload: None,
        })
        .await;

    let update_result = collection
        .update_request_entry(UpdateRequestEntryInput {
            id: first_id,
            name: Some("second".to_string()),
        })
        .await;

    assert!(matches!(
        update_result,
        OperationResult::Err(OperationError::AlreadyExists { .. })
    ));
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_entry_nonexistent() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();
    let request_name_new = format!("{request_name}_new");

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf() == PathBuf::from("requests").join(request_folder_name(&request_name))
        })
        .unwrap()
        .1;

    // Remove the entry first and try renaming
    let _ = collection
        .delete_request_entry(DeleteRequestEntryInput { id })
        .await
        .unwrap();

    // Wait for the deletion task to complete
    tokio::time::sleep(Duration::from_millis(500)).await;

    let update_result = collection
        .update_request_entry(UpdateRequestEntryInput {
            id,
            name: Some(request_name_new.clone()),
        })
        .await;

    assert!(matches!(
        update_result,
        OperationResult::Err(OperationError::NotFound { .. })
    ));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_entry_fs_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let request_new_name = format!("{request_name}_new");

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            url: None,
            payload: None,
        })
        .await;
    let request_path = PathBuf::from("requests").join(request_folder_name(&request_name));

    let id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == request_path)
        .unwrap()
        .1;

    // Delete the request from filesystem
    tokio::fs::remove_dir_all(&collection_path.join(&request_path))
        .await
        .unwrap();

    let update_result = collection
        .update_request_entry(UpdateRequestEntryInput {
            id,
            name: Some(request_new_name.clone()),
        })
        .await;

    assert!(matches!(
        update_result,
        OperationResult::Err(OperationError::NotFound { .. })
    ));
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_entry_nested() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();
    let request_new_name = format!("{request_name}_new");
    let request_dir_name = random_request_dir_name();
    let request_dir_path = PathBuf::from("requests").join(&request_dir_name);
    let _ = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: request_dir_path.clone(),
        })
        .await
        .unwrap();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: request_dir_path.join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let request_id = create_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf() == request_dir_path.join(request_folder_name(&request_name))
        })
        .unwrap()
        .1;

    let update_result = collection
        .update_request_entry(UpdateRequestEntryInput {
            id: request_id,
            name: Some(request_new_name.clone()),
        })
        .await;

    let changed_paths = update_result.unwrap().changed_paths;
    assert_eq!(changed_paths.len(), 2);
    assert!(
        changed_paths
            .into_iter()
            .any(|(path, _id, kind)| path.to_path_buf()
                == request_dir_path.join(request_folder_name(&request_new_name))
                && kind == &PathChangeKind::Updated)
    );
    assert!(changed_paths.into_iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == request_dir_path
                .join(request_folder_name(&request_new_name))
                .join("get.sapic")
            && kind == &PathChangeKind::Updated
    }));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_entry_incorrect_entity_type() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();
    let new_name = format!("{request_dir_name}_new");

    let destination = PathBuf::from("requests").join(&request_dir_name);
    let create_request_dir_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: destination.clone(),
        })
        .await;

    let request_dir_id = create_request_dir_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == destination)
        .unwrap()
        .1;

    let update_request_result = collection
        .update_request_entry(UpdateRequestEntryInput {
            id: request_dir_id.clone(),
            name: Some(new_name.clone()),
        })
        .await;

    assert!(matches!(
        update_request_result,
        OperationResult::Err(OperationError::InvalidInput(..))
    ));
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
