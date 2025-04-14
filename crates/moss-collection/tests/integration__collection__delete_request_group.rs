use std::path::{Path, PathBuf};
use moss_collection::collection::OperationError;
use moss_collection::models::operations::{CreateRequestGroupInput, CreateRequestInput, DeleteRequestGroupInput};
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_group_name;
use crate::shared::{request_group_relative_path, set_up_test_collection};

mod shared;


#[tokio::test]
async fn delete_request_group_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();
    let expected_path = collection_path.join(
        request_group_relative_path(Path::new(&request_group_name))
    );

    collection.create_request_group(CreateRequestGroupInput {
        path: PathBuf::from(&request_group_name),
    }).await.unwrap();

    let delete_request_group_result = collection.delete_request_group(DeleteRequestGroupInput {
        path: PathBuf::from(&request_group_name)
    }).await;
    delete_request_group_result.unwrap();

    // Check folder is removed
    assert!(!expected_path.exists());

    // Clean up
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }

}

#[tokio::test]
async fn delete_request_group_empty_path() {
    let (collection_path, collection) = set_up_test_collection().await;

    let delete_request_group_output = collection.delete_request_group(
        DeleteRequestGroupInput {
            path: PathBuf::new()
        }
    ).await;

    assert!(delete_request_group_output.is_err());
}

#[tokio::test]
async fn delete_request_group_with_requests() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_group_name = random_request_group_name();

    // requests/outer_request
    // requests/group/inner_request
    collection.create_request_group(CreateRequestGroupInput {
        path: PathBuf::from(&request_group_name),
    }).await.unwrap();

    collection.create_request(CreateRequestInput {
        name: "inner_request".to_string(),
        relative_path: Some(PathBuf::from(&request_group_name)),
        url: None,
        payload: None,
    }).await.unwrap();
    collection.create_request(CreateRequestInput {
        name: "outer_request".to_string(),
        relative_path: None,
        url: None,
        payload: None,
    }).await.unwrap();

    let delete_request_group_result = collection.delete_request_group(
        DeleteRequestGroupInput {
            path: PathBuf::from(&request_group_name)
        }
    ).await;
    assert!(delete_request_group_result.is_ok());

    let expected_path = collection_path.join(
        request_group_relative_path(Path::new(&request_group_name))
    );
    // Check request group folder is removed
    assert!(!expected_path.exists());

    // Check deleting only the requests within the request group
    let requests = collection.list_requests().await.unwrap();
    assert_eq!(requests.0.len(), 1);
    assert_eq!(requests.0[0].name, "outer_request");

    // Clean up
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_request_group_fs_already_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();
    let expected_path = collection_path.join(
        request_group_relative_path(Path::new(&request_group_name))
    );
    collection.create_request_group(
        CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name),
        }
    ).await.unwrap();
    collection.create_request(
        CreateRequestInput {
            name: "request".to_string(),
            relative_path: Some(PathBuf::from(&request_group_name)),
            url: None,
            payload: None,
        }
    ).await.unwrap();

    // We delete the folder from the filesystem
    tokio::fs::remove_dir_all(expected_path).await.unwrap();

    let delete_request_group_result = collection.delete_request_group(
        DeleteRequestGroupInput {
            path: PathBuf::from(&request_group_name)
        }
    ).await;

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
    collection.create_request_group(CreateRequestGroupInput {
        path: PathBuf::from(&request_group_name),
    }).await.unwrap();

    // Create inner request group
    collection.create_request_group(CreateRequestGroupInput {
        path: PathBuf::from(&request_group_name).join("subfolder"),
    }).await.unwrap();

    // Create a request in the outer request group
    collection.create_request(CreateRequestInput {
        name: "outer_request".to_string(),
        relative_path: Some(PathBuf::from(&request_group_name)),
        url: None,
        payload: None,
    }).await.unwrap();

    // Create a request in the inner request group
    collection.create_request(CreateRequestInput {
        name: "inner_request".to_string(),
        relative_path: Some(PathBuf::from(&request_group_name).join("subfolder")),
        url: None,
        payload: None,
    }).await.unwrap();

    // Delete the inner request group
    let delete_request_group_output = collection.delete_request_group(
        DeleteRequestGroupInput {
            path: PathBuf::from(&request_group_name).join("subfolder")
        }
    ).await;
    assert!(delete_request_group_output.is_ok());

    let inner_request_group_path = collection_path.join(
        request_group_relative_path(&Path::new(&request_group_name).join("subfolder"))
    );
    let outer_request_group_path = collection_path.join(
        request_group_relative_path(Path::new(&request_group_name))
    );

    // Check deleting only the inner request group folder
    assert!(!inner_request_group_path.exists());
    assert!(outer_request_group_path.exists());

    // Check deleting only the requests within the inner request group
    let requests = collection.list_requests().await.unwrap();
    assert_eq!(requests.0.len(), 1);
    assert_eq!(requests.0[0].name, "outer_request");

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
        .map(|s| (format!("{s}{}", random_request_group_name()), s))
        .collect::<Vec<_>>();

    for name in request_group_name_list {

    }
}