mod shared;

use moss_collection::models::operations::{CreateRequestInput, DeleteRequestInput};
use moss_common::leased_slotmap::ResourceKey;
use moss_testutils::random_name::random_request_name;

use crate::shared::{request_relative_path, set_up_test_collection};

#[tokio::test]
async fn delete_request_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let expected_path = collection_path.join(request_relative_path(&request_name, None));
    let create_request_output = collection
        .create_request(CreateRequestInput {
            name: request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let delete_collection_result = collection
        .delete_request(DeleteRequestInput {
            key: create_request_output.key,
        })
        .await;
    assert!(delete_collection_result.is_ok());

    // Check folder is removed
    assert!(!expected_path.exists());

    // Check updating requests
    let requests = collection.list_requests().await.unwrap();
    assert!(requests.0.is_empty());

    // Clean up
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_request_nonexisten_key() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    collection
        .create_request(CreateRequestInput {
            name: request_name.to_string(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let nonexisten_key = ResourceKey::from(45677);
    let delete_collection_result_1 = collection
        .delete_request(DeleteRequestInput {
            key: nonexisten_key,
        })
        .await;
    assert!(delete_collection_result_1.is_err());

    // Clean up
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }
}

#[tokio::test]
async fn delete_request_fs_already_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let expected_path = collection_path.join(request_relative_path(&request_name, None));
    let create_request_output = collection
        .create_request(CreateRequestInput {
            name: request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    // Delete the request folder
    std::fs::remove_dir_all(expected_path).unwrap();

    let delete_collection_result = collection
        .delete_request(DeleteRequestInput {
            key: create_request_output.key,
        })
        .await;
    assert!(delete_collection_result.is_ok());

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert!(list_requests_output.0.is_empty());

    // Clean up
    {
        tokio::fs::remove_dir_all(collection_path).await.unwrap();
    }
}
