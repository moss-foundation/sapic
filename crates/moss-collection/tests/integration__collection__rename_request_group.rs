mod shared;

use moss_collection::collection::OperationError;
use moss_collection::models::operations::{
    CreateRequestGroupInput, CreateRequestInput, RenameRequestGroupInput,
};
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_group_name;
use std::path::{Path, PathBuf};

use crate::shared::{request_folder_name, request_group_relative_path, set_up_test_collection};

#[tokio::test]
async fn rename_request_group_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let old_name = random_request_group_name();
    let new_name = format!("{old_name}_new");

    collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap();

    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput {
            path: PathBuf::from(&old_name),
            new_name: new_name.clone(),
        })
        .await;

    assert!(rename_request_group_result.is_ok());

    // Check filesystem renaming
    let old_path = collection_path.join(request_group_relative_path(Path::new(&old_name)));
    let new_path = collection_path.join(request_group_relative_path(Path::new(&new_name)));
    assert!(!old_path.exists());
    assert!(new_path.exists());

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn rename_request_group_empty_name() {
    let (collection_path, collection) = set_up_test_collection().await;

    let old_name = random_request_group_name();
    let new_name = "".to_owned();
    collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap();

    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput {
            path: PathBuf::from(&old_name),
            new_name,
        })
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
    collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap();

    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput {
            path: PathBuf::from(&old_name),
            new_name,
        })
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
    collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&test_request_group_name),
        })
        .await
        .unwrap();

    // Try renaming the new request group into an existing request group name
    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput {
            path: PathBuf::from(&test_request_group_name),
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
    collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap();

    let mut current_name = old_name.clone();
    for char in FOLDERNAME_SPECIAL_CHARS {
        let new_request_group_name = format!("{old_name}{char}");
        collection
            .rename_request_group(RenameRequestGroupInput {
                path: PathBuf::from(&current_name),
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

    collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&old_name),
        })
        .await
        .unwrap();

    collection
        .create_request(CreateRequestInput {
            name: "outer_request".to_string(),
            relative_path: None,
            url: None,
            payload: None,
        })
        .await
        .unwrap();
    collection
        .create_request(CreateRequestInput {
            name: "inner_request".to_string(),
            relative_path: Some(PathBuf::from(&old_name)),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let rename_request_group_result = collection
        .rename_request_group(RenameRequestGroupInput {
            path: PathBuf::from(&old_name),
            new_name: new_name.clone(),
        })
        .await;

    assert!(rename_request_group_result.is_ok());

    // Check request group folder is renamed
    let old_path = collection_path.join(request_group_relative_path(Path::new(&old_name)));
    let new_path = collection_path.join(request_group_relative_path(Path::new(&new_name)));
    assert!(!old_path.exists());
    assert!(new_path.exists());

    // Check only modifying the requests within the request group
    let requests = collection.list_requests().await.unwrap();
    assert_eq!(requests.0.len(), 2);
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

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn rename_request_group_subfolder() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();

    // Create inner request group
    collection
        .create_request_group(CreateRequestGroupInput {
            path: PathBuf::from(&request_group_name).join("subfolder"),
        })
        .await
        .unwrap();

    // Create a request in the outer request group
    collection
        .create_request(CreateRequestInput {
            name: "outer_request".to_string(),
            relative_path: Some(PathBuf::from(&request_group_name)),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    // Create a request in the inner request group
    collection
        .create_request(CreateRequestInput {
            name: "inner_request".to_string(),
            relative_path: Some(PathBuf::from(&request_group_name).join("subfolder")),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    // Rename the inner request group
    let rename_request_group_output = collection
        .rename_request_group(RenameRequestGroupInput {
            path: PathBuf::from(&request_group_name).join("subfolder"),
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

    // Check modifying only the requests within the inner request group
    let requests = collection.list_requests().await.unwrap();
    dbg!(&requests);
    assert_eq!(requests.0.len(), 2);
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

    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}
