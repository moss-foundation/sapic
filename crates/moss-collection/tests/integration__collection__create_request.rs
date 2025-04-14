mod shared;

use moss_collection::collection::OperationError;
use moss_collection::models::operations::{
    CreateRequestInput, CreateRequestProtocolSpecificPayload,
};
use moss_collection::models::types::{HttpMethod, RequestInfo, RequestProtocol};

use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_request_name};
use std::path::{Path, PathBuf};
use moss_fs::utils::encode_directory_name;
use crate::shared::{request_folder_name, request_relative_path, set_up_test_collection};

#[tokio::test]
async fn create_request_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();

    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: request_name.to_string(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await;

    assert!(create_request_result.is_ok());

    // Check creating the request folder and its sapic spec file
    let expected_request_path = collection_path.join(request_relative_path(&request_name, None));
    let expected_request_spec_path = expected_request_path.join("get.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    let create_request_output = create_request_result.unwrap();

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestInfo {
            key: create_request_output.key,
            name: request_name.to_string(),
            relative_path_from_requests_dir: PathBuf::from(request_folder_name(&request_name)),
            order: None,
            typ: RequestProtocol::Http(HttpMethod::Get),
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
        Err(OperationError::RequestAlreadyExists { .. })
    ));

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name_list = FILENAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_request_name()))
        .collect::<Vec<String>>();

    for name in request_name_list {
        let create_request_result = collection
            .create_request(CreateRequestInput {
                name: name.clone(),
                relative_path: None,
                url: None,
                payload: None,
            })
            .await;

        // Check creating the request folder and its sapic spec file with proper encoding

        let expected_request_path = collection_path.join(request_relative_path(&name, None));
        let expected_request_spec_path = expected_request_path.join("get.sapic");

        assert!(create_request_result.is_ok());
        assert!(expected_request_path.exists());
        assert!(expected_request_spec_path.exists());

        let create_request_output = create_request_result.unwrap();

        // Check updating requests
        let list_requests_output = collection.list_requests().await.unwrap();
        assert!(list_requests_output.0.iter().any(|info| info
            == &RequestInfo {
                key: create_request_output.key,
                name: name.clone(),
                relative_path_from_requests_dir: PathBuf::from(request_folder_name(&name)),
                order: None,
                typ: RequestProtocol::Http(HttpMethod::Get),
            }));
    }
    // {
    //     tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    // }
}

#[tokio::test]
async fn create_request_with_relative_path() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: request_name.clone(),
            relative_path: Some(PathBuf::from("relative")),
            url: None,
            payload: None,
        })
        .await;

    assert!(create_request_result.is_ok());

    // Check creating the request folder and sapic spec file
    let expected_request_path =
        collection_path.join(request_relative_path(&request_name, Some(Path::new("relative"))));
    let expected_request_spec_path = expected_request_path.join("get.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    let create_request_output = create_request_result.unwrap();

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestInfo {
            key: create_request_output.key,
            name: request_name.clone(),
            relative_path_from_requests_dir: PathBuf::from("relative")
                .join(request_folder_name(&request_name)),
            order: None,
            typ: RequestProtocol::Http(HttpMethod::Get),
        }
    );

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_with_special_chars_in_relative_path() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: request_name.clone(),
            relative_path: Some(PathBuf::from("rela.tive")),
            url: None,
            payload: None,
        })
        .await;

    assert!(create_request_result.is_ok());

    // Check creating the request folder and sapic spec file
    let expected_request_path =
        collection_path.join(request_relative_path(&request_name, Some(Path::new("rela.tive"))));
    let expected_request_spec_path = expected_request_path.join("get.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    let create_request_output = create_request_result.unwrap();

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestInfo {
            key: create_request_output.key,
            name: request_name.clone(),
            relative_path_from_requests_dir: PathBuf::from(encode_directory_name("rela.tive"))
                .join(request_folder_name(&request_name)),
            order: None,
            typ: RequestProtocol::Http(HttpMethod::Get),
        }
    );

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_with_different_methods() {
    let (collection_path, collection) = set_up_test_collection().await;

    let testcases = vec![
        (HttpMethod::Get, "get.sapic", RequestProtocol::Http(HttpMethod::Get)),
        (HttpMethod::Post, "post.sapic", RequestProtocol::Http(HttpMethod::Post)),
        (HttpMethod::Put, "put.sapic", RequestProtocol::Http(HttpMethod::Put)),
        (HttpMethod::Delete, "delete.sapic", RequestProtocol::Http(HttpMethod::Delete)),
    ];
    
    for (method, expected_spec_filename, protocol) in testcases {
        let request_name = random_request_name();
        let create_request_result = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                url: None,
                payload: Some(
                    CreateRequestProtocolSpecificPayload::Http {
                        method,
                        query_params: vec![],
                        path_params: vec![],
                        headers: vec![],
                        body: None,
                    }
                )
            }).await;

        assert!(create_request_result.is_ok());

        // Check creating the request folder and sapic spec file
        let expected_request_path = collection_path.join(request_relative_path(&request_name, None));
        let expected_request_spec_path = expected_request_path.join(expected_spec_filename);
        assert!(expected_request_path.exists());
        assert!(expected_request_spec_path.exists());

        let create_request_output = create_request_result.unwrap();
        let list_requests_output = collection.list_requests().await.unwrap();

        assert!(list_requests_output.0.iter().any(|info| info == &RequestInfo {
            key: create_request_output.key,
            name: request_name.clone(),
            relative_path_from_requests_dir: PathBuf::from(request_folder_name(&request_name)),
            order: None,
            typ: protocol.clone(),
        }));
    }

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_with_payload() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

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

    // Check creating the request folder and sapic spec file
    let expected_request_path = collection_path.join(request_relative_path(&request_name, None));
    let expected_request_spec_path = expected_request_path.join("post.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    let create_request_output = create_request_result.unwrap();
    let list_requests_output = collection.list_requests().await.unwrap();

    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestInfo {
            key: create_request_output.key,
            name: request_name.clone(),
            relative_path_from_requests_dir: PathBuf::from(request_folder_name(&request_name)),
            order: None,
            typ: RequestProtocol::Http(HttpMethod::Post),
        }
    );

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}
