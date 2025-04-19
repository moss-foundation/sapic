mod shared;

use moss_collection::collection::OperationError;
use moss_collection::models::operations::CreateRequestGroupInput;
use moss_collection::models::types::RequestNodeInfo;
use moss_fs::utils::{encode_name, encode_path};
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_group_name;
use std::path::{Path, PathBuf};

use crate::shared::{request_group_relative_path, set_up_test_collection};

#[tokio::test]
async fn create_request_group_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();

    let create_request_group_result = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name),
        })
        .await;
    assert!(create_request_group_result.is_ok());

    // Check creating the request group folder
    let expected_path =
        collection_path.join(request_group_relative_path(Path::new(&request_group_name)));
    assert!(expected_path.exists());

    // Check updating request nodes
    let create_request_group_output = create_request_group_result.unwrap();
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Group {
            key: create_request_group_output.key,
            name: request_group_name.to_string(),
            path: PathBuf::from(request_group_name),
            order: None,
        }
    );

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_group_empty_path() {
    let (_collection_path, collection) = set_up_test_collection().await;

    let create_request_group_result = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::new(),
        })
        .await;

    assert!(matches!(
        create_request_group_result,
        Err(OperationError::Validation(..))
    ))
}

#[tokio::test]
async fn create_request_group_already_exists() {
    let (_collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();
    collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name),
        })
        .await
        .unwrap();

    let create_request_group_result = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name),
        })
        .await;

    assert!(matches!(
        create_request_group_result,
        Err(OperationError::AlreadyExists { .. })
    ))
}

#[tokio::test]
async fn create_request_group_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{s}{}", random_request_group_name()))
        .collect::<Vec<_>>();

    for name in request_group_name_list {
        let create_request_group_result = collection
            .create_request_group(CreateRequestGroupInput {
                path: PathBuf::from(&name),
            })
            .await;
        assert!(create_request_group_result.is_ok());
        // Check creating the request group folder with proper encoding
        let expected_path = collection_path.join(request_group_relative_path(Path::new(&name)));
        assert!(expected_path.exists());

        // Check updating request nodes
        let key = create_request_group_result.unwrap().key;
        let list_requests_output = collection.list_requests().await.unwrap();
        assert!(list_requests_output.0.iter().any(|request_group| {
            request_group
                == &RequestNodeInfo::Group {
                    key,
                    name: name.clone(),
                    path: PathBuf::from(encode_name(&name)),
                    order: None,
                }
        }));
    }

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_group_nested_folder() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_group_name = random_request_group_name();

    let create_request_group_result = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name).join("inner"),
        })
        .await;
    assert!(create_request_group_result.is_ok());

    // Check creating the nested request group folder
    let expected_path = collection_path.join(request_group_relative_path(
        &Path::new(&request_group_name).join("inner"),
    ));
    assert!(expected_path.exists());

    // Check updating request nodes
    let key = create_request_group_result.unwrap().key;
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Group {
            key,
            name: "inner".to_string(),
            path: PathBuf::from(&request_group_name).join("inner"),
            order: None,
        }
    );

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}
