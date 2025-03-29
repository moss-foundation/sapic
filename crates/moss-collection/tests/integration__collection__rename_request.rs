use std::path::PathBuf;
use moss_collection::collection::OperationError;
use moss_collection::models::operations::collection_operations::{CreateRequestInput, RenameRequestInput, RequestInfo};
use crate::shared::{random_request_name, request_relative_path, request_folder_name, set_up_test_collection, SPECIAL_CHARS};

mod shared;
#[tokio::test]
async fn rename_request_success() {
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

    let new_request_name = random_request_name();
    let result = collection.rename_request(
        RenameRequestInput {
            key,
            new_name: new_request_name.clone(),
        }
    ).await;
    assert!(result.is_ok());

    // Check filesystem rename
    let expected_path = collection_path.join(request_relative_path(&new_request_name, None));
    assert!(expected_path.exists());
    assert!(!old_path.exists());

    // Check updating requests
    let requests = collection.list_requests().await.unwrap();
    assert_eq!(requests.0.len(), 1);
    assert_eq!(requests.0[0], RequestInfo {
        key,
        name: new_request_name.clone(),
        request_dir_relative_path: PathBuf::from(request_folder_name(&new_request_name)),

        order: None,
        typ: Default::default(),
    });

    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}

#[tokio::test]
async fn rename_request_empty_name() {
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

    let new_name = "".to_string();
    let result = collection.rename_request(
        RenameRequestInput {
            key,
            new_name,
        }
    ).await;

    assert!(matches!(result, Err(OperationError::Validation(_))));

    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}

#[tokio::test]
async fn rename_request_unchanged() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let key = collection.create_request(
        CreateRequestInput {
            name: request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        }
    ).await.unwrap().key;

    let new_name = request_name;
    let result = collection.rename_request(
        RenameRequestInput {
            key,
            new_name,
        }
    ).await;

    assert!(result.is_ok());

    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}

#[tokio::test]
async fn rename_request_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;

    let existing_request_name = random_request_name();
    // Create an existing request
    collection.create_request(
        CreateRequestInput {
            name: existing_request_name.to_string(),

            relative_path: None,
            url: None,
            payload: None,
        }
    ).await.unwrap();

    let new_request_name = random_request_name();
    // Create a request to test renaming
    let key = collection.create_request(
        CreateRequestInput {
            name: new_request_name,

            relative_path: None,
            url: None,
            payload: None,
        }
    ).await.unwrap().key;

    // Try renaming the new request to an existing request name
    let result = collection.rename_request(
        RenameRequestInput {
            key,
            new_name: existing_request_name
        }
    ).await;
    assert!(matches!(result, Err(OperationError::AlreadyExists {..})));

    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}

#[tokio::test]
async fn rename_request_special_chars() {
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

    for char in SPECIAL_CHARS {
        let new_request_name = format!("{request_name}{char}");
        collection.rename_request(
            RenameRequestInput {
                key,
                new_name: new_request_name.clone(),
            }
        ).await.unwrap();

        // Checking updating requests
        let requests = collection.list_requests().await.unwrap();
        assert_eq!(requests.0.len(), 1);
        assert_eq!(requests.0[0], RequestInfo {
            key,
            request_dir_relative_path: PathBuf::from(request_folder_name(&new_request_name)),
            name: new_request_name,
            order: None,
            typ: Default::default(),
        });
    }

    {
        std::fs::remove_dir_all(&collection_path).unwrap()
    }
}