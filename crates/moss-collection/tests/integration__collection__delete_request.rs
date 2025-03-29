use moss_collection::models::operations::collection_operations::{CreateRequestInput, DeleteRequestInput};
use crate::shared::{random_request_name, request_relative_path, set_up_test_collection};

mod shared;


#[tokio::test]
async fn delete_request_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let expected_path = collection_path.join(request_relative_path(&request_name, None));
    let key = collection.create_request(
        CreateRequestInput {
            name: request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        }
    ).await.unwrap().key;
    let result = collection.delete_request(
        DeleteRequestInput {
            key
        }
    ).await;
    assert!(result.is_ok());

    // Check folder is removed
    assert!(!expected_path.exists());

    // Check updating requests
    let requests = collection.list_requests().await.unwrap();
    assert!(requests.0.is_empty());

    {
        std::fs::remove_dir_all(collection_path).unwrap();
    }
}

#[tokio::test]
async fn delete_request_nonexisten_key() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let old_path = collection_path.join(request_relative_path(&request_name, None));
    let key = collection.create_request(
        CreateRequestInput {
            name: request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        }
    ).await.unwrap().key;
    collection.delete_request(
        DeleteRequestInput {
            key
        }
    ).await.unwrap();
    let result = collection.delete_request(
        DeleteRequestInput {
            key
        }
    ).await;
    assert!(result.is_err());

    {
        std::fs::remove_dir_all(collection_path).unwrap();
    }
}

#[tokio::test]
async fn delete_request_fs_already_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let expected_path = collection_path.join(request_relative_path(&request_name, None));
    let key = collection.create_request(
        CreateRequestInput {
            name: request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        }
    ).await.unwrap().key;

    // Delete the request folder
    std::fs::remove_dir_all(expected_path).unwrap();

    let result = collection.delete_request(
        DeleteRequestInput {
            key
        }
    ).await;
    assert!(result.is_ok());

    // Check updating requests
    let requests = collection.list_requests().await.unwrap();
    assert!(requests.0.is_empty());

    {
        std::fs::remove_dir_all(collection_path).unwrap();
    }
}