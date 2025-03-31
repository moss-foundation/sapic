mod shared;

use moss_collection::collection::OperationError;
use moss_collection::models::collection::{HttpRequestType, RequestType};
use moss_collection::models::operations::collection_operations::{
    CreateRequestInput, CreateRequestProtocolSpecificPayload, RequestInfo,
};
use moss_collection::models::types::request_types::HttpMethod;
use moss_testutils::{fs_specific::SPECIAL_CHARS, random_name::random_request_name};
use std::path::PathBuf;

use crate::shared::{request_folder_name, request_relative_path, set_up_test_collection};

#[tokio::test]
async fn create_request_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let expected_path = collection_path.join(request_relative_path(&request_name, None));
    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: request_name.to_string(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await;

    assert!(create_request_result.is_ok());
    assert!(expected_path.exists());

    let create_request_output = create_request_result.unwrap();

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestInfo {
            key: create_request_output.key,
            name: request_name.to_string(),
            request_dir_relative_path: PathBuf::from(request_folder_name(&request_name)),
            order: None,
            typ: Default::default(),
        }
    );

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_empty_name() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = "".to_string();
    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: request_name.to_string(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await;

    assert!(matches!(
        create_request_result,
        Err(OperationError::Validation(_))
    ));

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    collection
        .create_request(CreateRequestInput {
            name: request_name.clone(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: request_name.clone(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await;

    assert!(matches!(
        create_request_result,
        Err(OperationError::AlreadyExists { .. })
    ));

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name_list = SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_request_name()))
        .collect::<Vec<String>>();

    for name in request_name_list {
        let expected_path = collection_path.join(request_relative_path(&name, None));
        dbg!(&expected_path);

        let create_request_result = collection
            .create_request(CreateRequestInput {
                name: name.clone(),
                relative_path: None,
                url: None,
                payload: None,
            })
            .await;

        assert!(create_request_result.is_ok());
        assert!(expected_path.exists());

        let create_request_output = create_request_result.unwrap();

        // Check updating requests
        let list_requests_output = collection.list_requests().await.unwrap();
        assert!(list_requests_output.0.iter().any(|info| info
            == &RequestInfo {
                key: create_request_output.key,
                name: name.clone(),
                request_dir_relative_path: PathBuf::from(request_folder_name(&name)),
                order: None,
                typ: Default::default(),
            }));
    }
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_with_relative_path() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let expected_path =
        collection_path.join(request_relative_path(&request_name, Some("relative")));
    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: request_name.clone(),
            relative_path: Some(PathBuf::from("relative")),
            url: None,
            payload: None,
        })
        .await;

    assert!(create_request_result.is_ok());
    assert!(expected_path.exists());

    let create_request_output = create_request_result.unwrap();

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestInfo {
            key: create_request_output.key,
            name: request_name.clone(),
            request_dir_relative_path: PathBuf::from("relative")
                .join(request_folder_name(&request_name)),
            order: None,
            typ: Default::default(),
        }
    );

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_with_payload() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();
    let expected_path = collection_path.join(request_relative_path(&request_name, None));

    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: request_name.clone(),
            relative_path: None,
            url: None,
            payload: Some(CreateRequestProtocolSpecificPayload::Http {
                method: HttpMethod::Post,
                query_params: vec![],
                path_params: vec![],
                headers: vec![],
                body: None,
            }),
        })
        .await;

    assert!(create_request_result.is_ok());
    assert!(expected_path.exists());

    let create_request_output = create_request_result.unwrap();
    let list_requests_output = collection.list_requests().await.unwrap();

    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestInfo {
            key: create_request_output.key,
            name: request_name.clone(),
            request_dir_relative_path: PathBuf::from(request_folder_name(&request_name)),
            order: None,
            typ: RequestType::Http(HttpRequestType::Post),
        }
    );

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}
