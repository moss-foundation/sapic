mod shared;

use crate::shared::{request_folder_name, request_group_relative_path, set_up_test_collection};
use moss_collection::models::operations::{
    CreateRequestGroupInput, CreateRequestInput, DeleteRequestGroupInput,
};
use moss_common::leased_slotmap::ResourceKey;
use moss_models::collection::types::{HttpMethod, RequestNodeInfo, RequestProtocol};
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::{random_request_group_name, random_request_name};
use std::path::{Path, PathBuf};

#[tokio::test]
async fn delete_request_group_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();
    let expected_path =
        collection_path.join(request_group_relative_path(Path::new(&request_group_name)));

    let key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name),
        })
        .await
        .unwrap()
        .key;

    let delete_request_group_result = collection
        .delete_request_group(DeleteRequestGroupInput { key })
        .await;
    delete_request_group_result.unwrap();

    // Check folder is removed
    assert!(!expected_path.exists());

    // Check updating request nodes
    let list_requests_output = collection.list_requests().await.unwrap();
    assert!(list_requests_output.0.is_empty());

    // Clean up
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_request_group_nonexistent_key() {
    let (_collection_path, collection) = set_up_test_collection().await;

    let key = ResourceKey::from(42);
    let delete_request_group_output = collection
        .delete_request_group(DeleteRequestGroupInput { key })
        .await;

    assert!(delete_request_group_output.is_err());
}

#[tokio::test]
async fn delete_request_group_with_requests() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_group_name = random_request_group_name();

    // requests/outer_request
    // requests/group/inner_request
    let group_key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name),
        })
        .await
        .unwrap()
        .key;

    let inner_request_key = collection
        .create_request(CreateRequestInput {
            name: "inner_request".to_string(),
            relative_path: Some(PathBuf::from(&request_group_name)),
            url: None,
            payload: None,
        })
        .await
        .unwrap()
        .key;

    let outer_request_key = collection
        .create_request(CreateRequestInput {
            name: "outer_request".to_string(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap()
        .key;

    let delete_request_group_result = collection
        .delete_request_group(DeleteRequestGroupInput { key: group_key })
        .await;
    assert!(delete_request_group_result.is_ok());

    let expected_path =
        collection_path.join(request_group_relative_path(Path::new(&request_group_name)));
    // Check request group folder is removed
    assert!(!expected_path.exists());

    // Check deleting the group and inner request nodes
    let requests = collection.list_requests().await.unwrap();
    assert_eq!(requests.0.len(), 1);
    assert_eq!(requests.0[0].name(), "outer_request");

    // Clean up
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_request_group_fs_already_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();
    let expected_path =
        collection_path.join(request_group_relative_path(Path::new(&request_group_name)));

    let group_key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name),
        })
        .await
        .unwrap()
        .key;

    let request_key = collection
        .create_request(CreateRequestInput {
            name: "request".to_string(),
            relative_path: Some(PathBuf::from(&request_group_name)),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    // We delete the folder from the filesystem
    tokio::fs::remove_dir_all(expected_path).await.unwrap();

    let delete_request_group_result = collection
        .delete_request_group(DeleteRequestGroupInput { key: group_key })
        .await;

    assert!(delete_request_group_result.is_ok());

    // It should still update the in-memory `requests` map
    let list_requests_output = collection.list_requests().await.unwrap();
    assert!(list_requests_output.0.is_empty());

    // Clean up
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_request_group_subfolder() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();

    // Create outer request group
    let outer_group_key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name),
        })
        .await
        .unwrap()
        .key;

    // Create inner request group
    let inner_group_key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name).join("subfolder"),
        })
        .await
        .unwrap()
        .key;

    // Create a request in the outer request group
    let outer_request_key = collection
        .create_request(CreateRequestInput {
            name: "outer_request".to_string(),
            relative_path: Some(PathBuf::from(&request_group_name)),
            url: None,
            payload: None,
        })
        .await
        .unwrap()
        .key;

    // Create a request in the inner request group
    let inner_request_key = collection
        .create_request(CreateRequestInput {
            name: "inner_request".to_string(),
            relative_path: Some(PathBuf::from(&request_group_name).join("subfolder")),
            url: None,
            payload: None,
        })
        .await
        .unwrap()
        .key;

    // Delete the inner request group
    let delete_request_group_output = collection
        .delete_request_group(DeleteRequestGroupInput {
            key: inner_group_key,
        })
        .await;
    assert!(delete_request_group_output.is_ok());

    let inner_request_group_path = collection_path.join(request_group_relative_path(
        &Path::new(&request_group_name).join("subfolder"),
    ));
    let outer_request_group_path =
        collection_path.join(request_group_relative_path(Path::new(&request_group_name)));

    // Check deleting only the inner request group folder
    assert!(!inner_request_group_path.exists());
    assert!(outer_request_group_path.exists());

    // Check deleting only the inner group and inner request nodes
    let requests = collection.list_requests().await.unwrap();
    assert_eq!(requests.0.len(), 2);
    assert!(requests.0.iter().any(|entity| {
        entity
            == &RequestNodeInfo::Group {
                key: outer_group_key,
                name: request_group_name.clone(),
                path: PathBuf::from(&request_group_name),
                order: None,
            }
    }));
    assert!(requests.0.iter().any(|entity| {
        entity
            == &RequestNodeInfo::Request {
                key: outer_request_key,
                name: "outer_request".to_string(),
                path: PathBuf::from(&request_group_name).join(request_folder_name("outer_request")),
                order: None,
                protocol: RequestProtocol::Http(HttpMethod::Get),
            }
    }));

    // Cleanup
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_request_group_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_group_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| (format!("{s}{}", random_request_group_name())))
        .collect::<Vec<_>>();

    for name in request_group_name_list {
        let key = collection
            .create_request_group(CreateRequestGroupInput {
                path: PathBuf::from(&name),
            })
            .await
            .unwrap()
            .key;

        // FIXME: We will pass the resource key instead of unencoded path once implemented
        let delete_request_group_output = collection
            .delete_request_group(DeleteRequestGroupInput { key })
            .await;

        assert!(delete_request_group_output.is_ok());

        // Check the request group folder is deleted
        let expected_path = collection_path.join(request_group_relative_path(Path::new(&name)));
        assert!(!expected_path.exists());

        // Check the request group node is deleted
        let request_nodes = collection.list_requests().await.unwrap();
        assert!(request_nodes.0.is_empty());
    }

    // Clean up
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_request_group_incorrect_entity_type() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let request_key = collection
        .create_request(CreateRequestInput {
            name: request_name,
            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap()
        .key;

    let result = collection
        .delete_request_group(DeleteRequestGroupInput { key: request_key })
        .await;

    assert!(result.is_err());

    {
        std::fs::remove_dir_all(collection_path).unwrap();
    }
}
