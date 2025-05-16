mod shared;

use crate::shared::{random_request_dir_name, request_folder_name, set_up_test_collection};
use moss_collection::models::operations::{
    CreateRequestDirEntryInput, CreateRequestEntryInput, DeleteRequestDirEntryInput,
    UpdateRequestDirEntryInput,
};
use moss_collection::models::types::PathChangeKind;
use moss_common::api::{OperationError, OperationResult};
use moss_common::sanitized::SanitizedName;
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_name;
use std::path::PathBuf;
use std::time::Duration;

#[tokio::test]
async fn update_request_dir_entry_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let destination = PathBuf::from("requests").join("test");
    let new_destination = PathBuf::from("requests").join("new_name");
    let create_request_dir_entry_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: destination.clone(),
        })
        .await
        .unwrap();

    let dir_id = create_request_dir_entry_result
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == destination)
        .unwrap()
        .1;

    let update_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
            id: dir_id,
            name: Some("new_name".to_string()),
        })
        .await;

    let changed_paths = update_result.unwrap().changed_paths;
    assert_eq!(changed_paths.len(), 1);
    assert!(changed_paths.into_iter().any(|(path, id, kind)| {
        path.to_path_buf() == new_destination && id == &dir_id && kind == &PathChangeKind::Updated
    }));

    assert!(!collection_path.join(&destination).exists());
    assert!(collection_path.join(&new_destination).exists());

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_dir_entry_no_change() {
    let (collection_path, collection) = set_up_test_collection().await;
    let destination = PathBuf::from("requests").join("test");

    let create_request_dir_entry_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: destination.clone(),
        })
        .await
        .unwrap();

    let dir_id = create_request_dir_entry_result
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == destination)
        .unwrap()
        .1;

    let update_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
            id: dir_id.clone(),
            name: None,
        })
        .await;

    assert!(update_result.unwrap().changed_paths.is_empty());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_dir_entry_same_name() {
    let (collection_path, collection) = set_up_test_collection().await;
    let destination = PathBuf::from("requests").join("test");

    let create_request_dir_entry_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: destination.clone(),
        })
        .await
        .unwrap();

    let dir_id = create_request_dir_entry_result
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == destination)
        .unwrap()
        .1;

    let update_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
            id: dir_id.clone(),
            name: Some("test".to_string()),
        })
        .await;

    assert!(update_result.unwrap().changed_paths.is_empty());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_dir_entry_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;
    let first_destination = PathBuf::from("requests").join("first");
    let second_destination = PathBuf::from("requests").join("second");

    let create_request_dir_entry_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: first_destination.clone(),
        })
        .await;

    let first_id = create_request_dir_entry_result
        .unwrap()
        .changed_paths
        .iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == first_destination)
        .unwrap()
        .1;

    let _ = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: second_destination.clone(),
        })
        .await;

    let update_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
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
async fn update_request_dir_entry_nonexistent() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();
    let request_dir_new_name = format!("{request_dir_name}_new");
    let destination = PathBuf::from("requests").join(&request_dir_name);
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

    // Remove the dir entry first and try to update it
    let _ = collection
        .delete_request_dir_entry(DeleteRequestDirEntryInput { id: id.clone() })
        .await
        .unwrap();

    // Wait for the deletion task to complete
    tokio::time::sleep(Duration::from_millis(500)).await;

    let update_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
            id: id.clone(),
            name: Some(request_dir_new_name.clone()),
        })
        .await;

    assert!(matches!(
        update_result,
        OperationResult::Err(OperationError::NotFound { .. })
    ));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_dir_entry_fs_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();
    let request_dir_new_name = format!("{request_dir_name}_new");

    let destination = PathBuf::from("requests").join(&request_dir_name);
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

    // Delete the request dir from the filesystem
    tokio::fs::remove_dir_all(collection_path.join(&destination))
        .await
        .unwrap();

    let update_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
            id,
            name: Some(request_dir_new_name.clone()),
        })
        .await;

    assert!(matches!(
        update_result,
        OperationResult::Err(OperationError::NotFound { .. })
    ));
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
#[tokio::test]
async fn update_request_dir_entry_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();
    let destination = PathBuf::from("requests").join(&request_dir_name);
    let create_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: destination.clone(),
        })
        .await;

    let dir_id = create_result
        .unwrap()
        .changed_paths
        .iter()
        .find(|(path, _id, _kind)| path.to_path_buf() == destination)
        .unwrap()
        .1;

    for char in FOLDERNAME_SPECIAL_CHARS {
        let new_name = format!("{char}{}", request_dir_name);
        let sanitized_name = SanitizedName::new(&new_name);
        let update_result = collection
            .update_request_dir_entry(UpdateRequestDirEntryInput {
                id: dir_id.clone(),
                name: Some(new_name.clone()),
            })
            .await;

        let changed_paths = update_result.unwrap().changed_paths;

        assert_eq!(changed_paths.len(), 1);
        assert!(changed_paths.into_iter().any(|(path, id, kind)| {
            path.to_path_buf() == PathBuf::from("requests").join(&sanitized_name)
                && id == &dir_id
                && kind == &PathChangeKind::Updated
        }));
        assert!(
            collection_path
                .join("requests")
                .join(&sanitized_name)
                .exists()
        );
    }

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_dir_entry_with_content() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();
    let request_dir_new_name = format!("{request_dir_name}_new");
    let request_name = random_request_name();

    let dir_destination = PathBuf::from("requests").join(&request_dir_name);
    let dir_new_destination = PathBuf::from("requests").join(&request_dir_new_name);
    // requests/dir -> requests/dir_new
    // requests/dir/request.request -> requests/dir_new/request.request
    // requests/dir/request.request/get.sapic -> requests/dir_new/request.request/get.sapic
    let create_dir_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: dir_destination.clone(),
        })
        .await;

    let dir_id = create_dir_result
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

    let update_dir_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
            id: dir_id.clone(),
            name: Some(request_dir_new_name.clone()),
        })
        .await;

    let changed_paths = update_dir_result.unwrap().changed_paths;

    assert_eq!(changed_paths.len(), 3);
    assert!(changed_paths.into_iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests").join(&request_dir_new_name)
            && kind == &PathChangeKind::Updated
    }));
    assert!(changed_paths.into_iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join(&request_dir_new_name)
                .join(request_folder_name(&request_name))
            && kind == &PathChangeKind::Updated
    }));
    assert!(changed_paths.into_iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join(&request_dir_new_name)
                .join(request_folder_name(&request_name))
                .join("get.sapic")
            && kind == &PathChangeKind::Updated
    }));

    assert!(collection_path.join(&dir_new_destination).exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_dir_entry_subfolder() {
    let (collection_path, collection) = set_up_test_collection().await;
    let outer_dir_name = random_request_dir_name();
    let inner_dir_name = random_request_dir_name();
    let inner_dir_new_name = format!("{inner_dir_name}_new");

    let _ = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: PathBuf::from("requests").join(&outer_dir_name),
        })
        .await
        .unwrap();

    let create_inner_dir_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: PathBuf::from("requests")
                .join(&outer_dir_name)
                .join(&inner_dir_name),
        })
        .await;

    let inner_dir_id = create_inner_dir_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf()
                == PathBuf::from("requests")
                    .join(&outer_dir_name)
                    .join(&inner_dir_name)
        })
        .unwrap()
        .1;

    let update_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
            id: inner_dir_id.clone(),
            name: Some(inner_dir_new_name.clone()),
        })
        .await
        .unwrap();

    assert_eq!(update_result.changed_paths.len(), 1);
    assert!(
        update_result
            .changed_paths
            .into_iter()
            .any(|(path, id, kind)| {
                path.to_path_buf()
                    == PathBuf::from("requests")
                        .join(&outer_dir_name)
                        .join(&inner_dir_new_name)
                    && id == &inner_dir_id
                    && kind == &PathChangeKind::Updated
            })
    );

    assert!(
        collection_path
            .join("requests")
            .join(&outer_dir_name)
            .join(&inner_dir_new_name)
            .exists()
    );
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_request_dir_entry_incorrect_entity_type() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();
    let request_name = random_request_name();
    let new_name = format!("{request_name}_new");

    let create_request_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests")
                .join(&request_dir_name)
                .join(&request_name),
            url: None,
            payload: None,
        })
        .await;

    let request_id = create_request_result
        .unwrap()
        .changed_paths
        .into_iter()
        .find(|(path, _id, _kind)| {
            path.to_path_buf()
                == PathBuf::from("requests")
                    .join(&request_dir_name)
                    .join(request_folder_name(&request_name))
        })
        .unwrap()
        .1;

    let update_request_dir_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
            id: request_id.clone(),
            name: Some(new_name.clone()),
        })
        .await;

    assert!(matches!(
        update_request_dir_result,
        OperationResult::Err(OperationError::InvalidInput(..))
    ));
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
