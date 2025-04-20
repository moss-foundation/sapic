mod shared;

use moss_collection::models::operations::{
    CreateRequestInput, CreateRequestProtocolSpecificPayload,
};
use moss_common::api::OperationError;
use moss_fs::utils::encode_name;
use moss_models::collection::types::{HttpMethod, RequestNodeInfo, RequestProtocol};
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_request_name};
use std::path::{Path, PathBuf};

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
        RequestNodeInfo::Request {
            key: create_request_output.key,
            name: request_name.to_string(),
            path: PathBuf::from(request_folder_name(&request_name)),
            order: None,
            protocol: RequestProtocol::Http(HttpMethod::Get),
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
            == &RequestNodeInfo::Request {
                key: create_request_output.key,
                name: name.clone(),
                path: PathBuf::from(request_folder_name(&name)),
                order: None,
                protocol: RequestProtocol::Http(HttpMethod::Get),
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
    let expected_request_path = collection_path.join(request_relative_path(
        &request_name,
        Some(Path::new("relative")),
    ));
    let expected_request_spec_path = expected_request_path.join("get.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    let create_request_output = create_request_result.unwrap();

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Request {
            key: create_request_output.key,
            name: request_name.clone(),
            path: PathBuf::from("relative").join(request_folder_name(&request_name)),
            order: None,
            protocol: RequestProtocol::Http(HttpMethod::Get),
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
    let expected_request_path = collection_path.join(request_relative_path(
        &request_name,
        Some(Path::new("rela.tive")),
    ));
    let expected_request_spec_path = expected_request_path.join("get.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    let create_request_output = create_request_result.unwrap();

    // Check updating requests
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Request {
            key: create_request_output.key,
            name: request_name.clone(),
            path: PathBuf::from(encode_name("rela.tive")).join(request_folder_name(&request_name)),
            order: None,
            protocol: RequestProtocol::Http(HttpMethod::Get),
        }
    );

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_http_get() {
    let (collection_path, collection) = set_up_test_collection().await;

    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: "get".to_string(),
            relative_path: None,
            url: None,
            payload: Some(CreateRequestProtocolSpecificPayload::Http {
                method: HttpMethod::Get,
                query_params: vec![],
                path_params: vec![],
                headers: vec![],
                body: None,
            }),
        })
        .await;

    assert!(create_request_result.is_ok());
    let key = create_request_result.unwrap().key;
    // Check creating the request folder and sapic spec file

    let expected_request_path = collection_path.join(request_relative_path(&"get", None));
    let expected_request_spec_path = expected_request_path.join("get.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    // Check updating request map
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Request {
            key,
            name: "get".to_string(),
            path: PathBuf::from("get.request"),
            order: None,
            protocol: RequestProtocol::Http(HttpMethod::Get),
        }
    );

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_http_post() {
    let (collection_path, collection) = set_up_test_collection().await;

    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: "post".to_string(),
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
    let key = create_request_result.unwrap().key;
    // Check creating the request folder and sapic spec file

    let expected_request_path = collection_path.join(request_relative_path(&"post", None));
    let expected_request_spec_path = expected_request_path.join("post.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    // Check updating request map
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Request {
            key,
            name: "post".to_string(),
            path: PathBuf::from("post.request"),
            order: None,
            protocol: RequestProtocol::Http(HttpMethod::Post),
        }
    );

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_http_put() {
    let (collection_path, collection) = set_up_test_collection().await;

    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: "put".to_string(),
            relative_path: None,
            url: None,
            payload: Some(CreateRequestProtocolSpecificPayload::Http {
                method: HttpMethod::Put,
                query_params: vec![],
                path_params: vec![],
                headers: vec![],
                body: None,
            }),
        })
        .await;

    assert!(create_request_result.is_ok());
    let key = create_request_result.unwrap().key;
    // Check creating the request folder and sapic spec file

    let expected_request_path = collection_path.join(request_relative_path(&"put", None));
    let expected_request_spec_path = expected_request_path.join("put.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    // Check updating request map
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Request {
            key,
            name: "put".to_string(),
            path: PathBuf::from("put.request"),
            order: None,
            protocol: RequestProtocol::Http(HttpMethod::Put),
        }
    );

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_http_delete() {
    let (collection_path, collection) = set_up_test_collection().await;

    let create_request_result = collection
        .create_request(CreateRequestInput {
            name: "delete".to_string(),
            relative_path: None,
            url: None,
            payload: Some(CreateRequestProtocolSpecificPayload::Http {
                method: HttpMethod::Delete,
                query_params: vec![],
                path_params: vec![],
                headers: vec![],
                body: None,
            }),
        })
        .await;

    assert!(create_request_result.is_ok());
    let key = create_request_result.unwrap().key;
    // Check creating the request folder and sapic spec file

    let expected_request_path = collection_path.join(request_relative_path(&"delete", None));
    let expected_request_spec_path = expected_request_path.join("delete.sapic");
    assert!(expected_request_path.exists());
    assert!(expected_request_spec_path.exists());

    // Check updating request map
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(list_requests_output.0.len(), 1);
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Request {
            key,
            name: "delete".to_string(),
            path: PathBuf::from("delete.request"),
            order: None,
            protocol: RequestProtocol::Http(HttpMethod::Delete),
        }
    );

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}
