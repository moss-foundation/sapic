mod shared;

use moss_collection::models::operations::{
    CreateRequestGroupInput, CreateRequestInput, RenameRequestInput,
};
use moss_collection::models::types::{HttpMethod, RequestNodeInfo, RequestProtocol};
use moss_common::api::OperationError;
use moss_testutils::random_name::random_request_group_name;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_request_name};
use std::path::{Path, PathBuf};

use crate::shared::{request_folder_name, request_relative_path, set_up_test_collection};

#[tokio::test]
async fn rename_request_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let old_path = collection_path.join(request_relative_path(&request_name, None));
    let create_request_output = collection
        .create_request_old(CreateRequestInput {
            name: request_name.to_string(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let new_request_name = random_request_name();
    let rename_request_result = collection
        .rename_request(RenameRequestInput {
            key: create_request_output.key,
            new_name: new_request_name.clone(),
        })
        .await;
    assert!(rename_request_result.is_ok());

    // Check filesystem rename
    let expected_path = collection_path.join(request_relative_path(&new_request_name, None));
    assert!(expected_path.exists());
    assert!(!old_path.exists());

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Request {
            key: create_request_output.key,
            name: new_request_name.clone(),
            path: PathBuf::from(request_folder_name(&new_request_name)),

            order: None,
            protocol: RequestProtocol::Http(HttpMethod::Get),
        }
    );

    // Clean up
    { tokio::fs::remove_dir_all(&collection_path).await.unwrap() }
}

#[tokio::test]
async fn rename_request_empty_name() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let create_request_output = collection
        .create_request_old(CreateRequestInput {
            name: request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let new_name = "".to_string();
    let rename_request_result = collection
        .rename_request(RenameRequestInput {
            key: create_request_output.key,
            new_name,
        })
        .await;

    assert!(matches!(
        rename_request_result,
        Err(OperationError::Validation(_))
    ));

    // Clean up
    { tokio::fs::remove_dir_all(&collection_path).await.unwrap() }
}

#[tokio::test]
async fn rename_request_unchanged() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let create_request_output = collection
        .create_request_old(CreateRequestInput {
            name: request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let new_name = request_name;
    let rename_request_result = collection
        .rename_request(RenameRequestInput {
            key: create_request_output.key,
            new_name,
        })
        .await;

    assert!(rename_request_result.is_ok());

    // Clean up
    { tokio::fs::remove_dir_all(&collection_path).await.unwrap() }
}

#[tokio::test]
async fn rename_request_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;

    let existing_request_name = random_request_name();
    // Create an existing request
    collection
        .create_request_old(CreateRequestInput {
            name: existing_request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let new_request_name = random_request_name();
    // Create a request to test renaming
    let create_request_output = collection
        .create_request_old(CreateRequestInput {
            name: new_request_name,

            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    // Try renaming the new request to an existing request name
    let rename_request_result = collection
        .rename_request(RenameRequestInput {
            key: create_request_output.key,
            new_name: existing_request_name,
        })
        .await;
    assert!(matches!(
        rename_request_result,
        Err(OperationError::AlreadyExists { .. })
    ));

    // Clean up
    { tokio::fs::remove_dir_all(&collection_path).await.unwrap() }
}

#[tokio::test]
async fn rename_request_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let create_request_output = collection
        .create_request_old(CreateRequestInput {
            name: request_name.to_string(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    for char in FILENAME_SPECIAL_CHARS {
        let new_request_name = format!("{request_name}{char}");
        collection
            .rename_request(RenameRequestInput {
                key: create_request_output.key,
                new_name: new_request_name.clone(),
            })
            .await
            .unwrap();

        // Checking updating requests
        let list_requests_output = collection.list_requests().await.unwrap();
        assert_eq!(list_requests_output.0.len(), 1);
        assert_eq!(
            list_requests_output.0[0],
            RequestNodeInfo::Request {
                key: create_request_output.key,
                path: PathBuf::from(request_folder_name(&new_request_name)),
                name: new_request_name,
                order: None,
                protocol: RequestProtocol::Http(HttpMethod::Get),
            }
        );
    }

    // Clean up
    { tokio::fs::remove_dir_all(&collection_path).await.unwrap() }
}

#[tokio::test]
async fn rename_request_with_relative_path() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let old_path = collection_path.join("requests").join(request_relative_path(
        &request_name,
        Some(Path::new("subfolder")),
    ));
    let create_request_output = collection
        .create_request_old(CreateRequestInput {
            name: request_name.to_string(),
            relative_path: Some(PathBuf::from("subfolder")),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let new_request_name = random_request_name();
    let rename_request_result = collection
        .rename_request(RenameRequestInput {
            key: create_request_output.key,
            new_name: new_request_name.clone(),
        })
        .await;
    assert!(rename_request_result.is_ok());

    // Check filesystem rename
    let expected_path = collection_path.join(request_relative_path(
        &new_request_name,
        Some(Path::new("subfolder")),
    ));
    assert!(expected_path.exists());
    assert!(!old_path.exists());

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Request {
            key: create_request_output.key,
            name: new_request_name.clone(),
            path: PathBuf::from("subfolder").join(request_folder_name(&new_request_name)),
            order: None,
            protocol: RequestProtocol::Http(HttpMethod::Get),
        }
    )
}

#[tokio::test]
async fn rename_request_incorrect_entity_type() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_group_name = random_request_group_name();

    let group_key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name),
        })
        .await
        .unwrap()
        .key;

    let result = collection
        .rename_request(RenameRequestInput {
            key: group_key,
            new_name: "new_name".to_string(),
        })
        .await;

    assert!(result.is_err());
    {
        std::fs::remove_dir_all(collection_path).unwrap();
    }
}
