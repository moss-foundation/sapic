mod shared;

use moss_collection::models::operations::{
    CreateRequestGroupInput, CreateRequestInput, RenameRequestGroupInput,
};
use moss_collection::models::types::RequestNodeInfo;
use moss_common::api::OperationError;
use moss_fs::utils::encode_name;
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::{random_request_group_name, random_request_name};
use std::path::{Path, PathBuf};

use crate::shared::{request_folder_name, request_group_relative_path, set_up_test_collection};

#[tokio::test]
async fn rename_request_group_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let old_name = random_request_group_name();
    let new_name = format!("{old_name}_new");

    let key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap()
        .key;

    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput {
            key,
            new_name: new_name.clone(),
        })
        .await;

    assert!(rename_request_group_result.is_ok());

    // Check filesystem renaming
    let old_path = collection_path.join(request_group_relative_path(Path::new(&old_name)));
    let new_path = collection_path.join(request_group_relative_path(Path::new(&new_name)));
    assert!(!old_path.exists());
    assert!(new_path.exists());

    // Check updating the request group node
    let list_requests_output = collection.list_requests().await.unwrap();
    assert_eq!(
        list_requests_output.0[0],
        RequestNodeInfo::Group {
            key,
            name: new_name.clone(),
            path: PathBuf::from(&new_name),
            order: None
        }
    );

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn rename_request_group_empty_name() {
    let (collection_path, collection) = set_up_test_collection().await;

    let old_name = random_request_group_name();
    let new_name = "".to_owned();
    let key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap()
        .key;

    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput { key, new_name })
        .await;

    assert!(matches!(
        rename_request_group_result,
        Err(OperationError::Validation { .. })
    ));

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn rename_request_group_unchanged() {
    let (collection_path, collection) = set_up_test_collection().await;

    let old_name = random_request_group_name();
    let new_name = old_name.clone();
    let key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap()
        .key;

    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput { key, new_name })
        .await;

    assert!(rename_request_group_result.is_ok());

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn rename_request_group_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;

    let existing_request_group_name = random_request_group_name();
    // Create an existing request group
    collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&existing_request_group_name),
        })
        .await
        .unwrap();

    let test_request_group_name = random_request_group_name();
    // Create a request group to test renaming
    let key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&test_request_group_name),
        })
        .await
        .unwrap()
        .key;

    // Try renaming the new request group into an existing request group name
    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput {
            key,
            new_name: existing_request_group_name.clone(),
        })
        .await;
    assert!(matches!(
        rename_request_group_result,
        Err(OperationError::AlreadyExists { .. })
    ));

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn rename_request_group_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;

    let old_name = random_request_group_name();
    let key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap()
        .key;

    let mut current_name = old_name.clone();
    for char in FOLDERNAME_SPECIAL_CHARS {
        let new_request_group_name = format!("{old_name}{char}");
        collection
            .rename_request_group(RenameRequestGroupInput {
                key,
                new_name: new_request_group_name.clone(),
            })
            .await
            .unwrap();
        let old_path = collection_path.join(request_group_relative_path(Path::new(&current_name)));
        let expected_path = collection_path.join(request_group_relative_path(Path::new(
            &new_request_group_name,
        )));
        assert!(!old_path.exists());
        assert!(expected_path.exists());

        // Check updating request group node
        let list_requests_output = collection.list_requests().await.unwrap();
        assert_eq!(
            list_requests_output.0[0],
            RequestNodeInfo::Group {
                key,
                name: new_request_group_name.clone(),
                path: PathBuf::from(encode_name(&new_request_group_name)),
                order: None
            }
        );

        current_name = new_request_group_name;
    }

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn rename_request_group_with_requests() {
    let (collection_path, collection) = set_up_test_collection().await;

    let old_name = random_request_group_name();
    let new_name = format!("{old_name}_new");
    // requests/outer_request
    // requests/group/inner_request

    let group_key = collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap()
        .key;

    let outer_request_key = collection
        .create_request_old(CreateRequestInput {
            name: "outer_request".to_string(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap()
        .key;
    let inner_request_key = collection
        .create_request_old(CreateRequestInput {
            name: "inner_request".to_string(),
            relative_path: Some(PathBuf::from(&old_name)),
            url: None,
            payload: None,
        })
        .await
        .unwrap()
        .key;

    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput {
            key: group_key,
            new_name: new_name.clone(),
        })
        .await;

    assert!(rename_request_group_result.is_ok());

    // Check request group folder is renamed
    let old_path = collection_path.join(request_group_relative_path(Path::new(&old_name)));
    let new_path = collection_path.join(request_group_relative_path(Path::new(&new_name)));
    assert!(!old_path.exists());
    assert!(new_path.exists());

    // Check only modifying the requests within the request group and the request group
    let requests = collection.list_requests().await.unwrap();
    assert!(requests
        .0
        .iter()
        .any(|request| request.name() == "outer_request"
            && request.path() == &PathBuf::from(request_folder_name("outer_request"))));
    assert!(requests
        .0
        .iter()
        .any(|request| request.name() == "inner_request"
            && request.path()
                == &PathBuf::from(&new_name).join(request_folder_name("inner_request"))));
    assert!(requests.0.iter().any(|request_group| {
        request_group
            == &RequestNodeInfo::Group {
                key: group_key,
                name: new_name.clone(),
                path: PathBuf::from(&new_name),
                order: None,
            }
    }));

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn rename_request_group_subfolder() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();

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
        .create_request_old(CreateRequestInput {
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
        .create_request_old(CreateRequestInput {
            name: "inner_request".to_string(),
            relative_path: Some(PathBuf::from(&request_group_name).join("subfolder")),
            url: None,
            payload: None,
        })
        .await
        .unwrap()
        .key;

    // Rename the inner request group
    let rename_request_group_output = collection
        .rename_request_group(RenameRequestGroupInput {
            key: inner_group_key,
            new_name: "subfolder_new".to_string(),
        })
        .await;
    assert!(rename_request_group_output.is_ok());

    let old_path = collection_path.join(request_group_relative_path(
        &Path::new(&request_group_name).join("subfolder"),
    ));
    let new_path = collection_path.join(request_group_relative_path(
        &Path::new(&request_group_name).join("subfolder_new"),
    ));

    // Check the folder is renamed
    assert!(!old_path.exists());
    assert!(new_path.exists());

    // Check modifying only the requests within the inner request group and the group node
    let requests = collection.list_requests().await.unwrap();
    assert!(requests
        .0
        .iter()
        .any(|request| request.name() == "outer_request"
            && request.path()
                == &PathBuf::from(&request_group_name).join(request_folder_name("outer_request"))));
    assert!(requests
        .0
        .iter()
        .any(|request| request.name() == "inner_request"
            && request.path()
                == &PathBuf::from(&request_group_name)
                    .join("subfolder_new")
                    .join(request_folder_name("inner_request"))));
    assert!(requests.0.iter().any(|request_group| {
        request_group
            == &RequestNodeInfo::Group {
                key: inner_group_key,
                name: "subfolder_new".to_string(),
                path: PathBuf::from(&request_group_name).join("subfolder_new"),
                order: None,
            }
    }));
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn rename_request_group_incorrect_entity_type() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let request_key = collection
        .create_request_old(CreateRequestInput {
            name: request_name,
            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap()
        .key;

    let result = collection
        .rename_request_group(RenameRequestGroupInput {
            key: request_key,
            new_name: "new name".to_string(),
        })
        .await;

    assert!(result.is_err());

    {
        std::fs::remove_dir_all(collection_path).unwrap();
    }
}
