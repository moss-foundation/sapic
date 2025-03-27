use std::path::PathBuf;
use moss_collection::collection::OperationError;
use moss_collection::models::collection::{HttpRequestType, RequestType};
use moss_collection::models::operations::collection_operations::{CreateRequestInput, CreateRequestProtocolSpecificPayload, RequestInfo};
use moss_collection::models::types::request_types::HttpMethod;
use moss_fs::utils::encode_directory_name;
use crate::shared::{random_collection_name, random_request_name, request_relative_path, request_folder_name, set_up_test_collection, SPECIAL_CHARS};

mod shared;


#[tokio::test]
async fn create_request_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let expected_path = collection_path.join(request_relative_path(&request_name, None));
    let output = collection.create_request(
        CreateRequestInput {
            name: request_name.to_string(),
            relative_path: None,
            url: None,
            payload: None,
        }
    ).await.unwrap();

    assert!(expected_path.exists());

    // Check updating requests
    let requests = collection.list_requests().await.unwrap();
    dbg!(&requests.0[0]);
    assert_eq!(requests.0.len(), 1);
    assert_eq!(requests.0[0], RequestInfo {
        key: output.key,
        name: request_name.to_string(),
        request_dir_relative_path: PathBuf::from(request_folder_name(&request_name)),
        order: None,
        typ: Default::default(),
    });

    // Clean up
    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}

#[tokio::test]
async fn create_request_empty_name() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = "".to_string();
    let result = collection.create_request(
        CreateRequestInput {
            name: request_name.to_string(),
            relative_path: None,
            url: None,
            payload: None,
        }
    ).await;

    assert!(matches!(result, Err(OperationError::Validation(_))));

    // Clean up
    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}

#[tokio::test]
async fn create_request_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    collection.create_request(
        CreateRequestInput {
            name: request_name.clone(),
            relative_path: None,
            url: None,
            payload: None,
        }
    ).await.unwrap();

    let result = collection.create_request(
        CreateRequestInput {
            name: request_name.clone(),
            relative_path: None,
            url: None,
            payload: None,
        }
    ).await;

    assert!(matches!(result, Err(OperationError::AlreadyExists {..})));
    // Clean up
    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}

#[tokio::test]
async fn create_request_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name_list =
        SPECIAL_CHARS
            .into_iter()
            .map(|s| format!("{}{s}", random_request_name()))
            .collect::<Vec<String>>();

    for name in request_name_list {
        let expected_path = collection_path.join(request_relative_path(&name, None));
        dbg!(&expected_path);

        let output = collection.create_request(
            CreateRequestInput {
                name: name.clone(),
                relative_path: None,
                url: None,
                payload: None,
            }
        ).await.unwrap();
        
        assert!(expected_path.exists());
        
        // Check updating requests
        let requests = collection.list_requests().await.unwrap();
        assert!(requests.0.iter().any(|info| info == &RequestInfo {
            key: output.key,
            name: name.clone(),
            request_dir_relative_path: PathBuf::from(request_folder_name(&name)),
            order: None,
            typ: Default::default(),
        }));
    }
    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}

#[tokio::test]
async fn create_request_with_relative_path() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let expected_path = collection_path.join(request_relative_path(&request_name, Some("relative")));
    let output = collection.create_request(
        CreateRequestInput {
            name: request_name.clone(),
            relative_path: Some(PathBuf::from("relative")),
            url: None,
            payload: None,
        }
    ).await.unwrap();

    assert!(expected_path.exists());
    // Check updating requests
    let requests = collection.list_requests().await.unwrap();
    assert_eq!(requests.0.len(), 1);
    assert_eq!(requests.0[0], RequestInfo {
        key: output.key,
        name: request_name.clone(),
        request_dir_relative_path: PathBuf::from("relative").join(request_folder_name(&request_name)),
        order: None,
        typ: Default::default(),
    });

    // Clean up
    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}

#[tokio::test]
async fn create_request_with_payload() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();
    let expected_path = collection_path.join(request_relative_path(&request_name, None));

    let output = collection.create_request(
        CreateRequestInput {
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
        }
    ).await.unwrap();
    assert!(expected_path.exists());
    let requests = collection.list_requests().await.unwrap();

    assert_eq!(requests.0.len(), 1);
    assert_eq!(requests.0[0], RequestInfo {
        key: output.key,
        name: request_name.clone(),
        request_dir_relative_path: PathBuf::from(request_folder_name(&request_name)),
        order: None,
        typ: RequestType::Http(HttpRequestType::Post),
    });

    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}
