use std::path::{Path, PathBuf};
use moss_collection::collection::OperationError;
use moss_collection::models::operations::CreateRequestGroupInput;
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_group_name;
use crate::shared::{request_group_relative_path, set_up_test_collection};

mod shared;


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

    // Check creating the request group folder and its sapic spec file
    let expected_path = collection_path.join(
        request_group_relative_path(Path::new(&request_group_name)),
    );
    let expected_spec_path = expected_path.join("folder.sapic");
    assert!(expected_path.exists());
    assert!(expected_spec_path.exists());

    // TODO: test the CreateRequestGroupOutput once implemented

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_request_group_empty_path() {
    let (collection_path, collection) = set_up_test_collection().await;

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
    let (collection_path, collection) = set_up_test_collection().await;

    let request_group_name = random_request_group_name();
    collection
        .create_request_group(
            CreateRequestGroupInput{
                path: PathBuf::from(&request_group_name),
            }
        )
        .await
        .unwrap();

    let create_request_group_result = collection
        .create_request_group(
            CreateRequestGroupInput {
                path: PathBuf::from(&request_group_name),
            }
        )
        .await;

    assert!(matches!(
        create_request_group_result,
        Err(OperationError::RequestGroupAlreadyExists {..})
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
            }).await;

        // Check creating the request group folder and its sapic spec file with proper encoding
        let expected_path = collection_path.join(request_group_relative_path(
            Path::new(&name),
        ));
        let expected_spec_path = expected_path.join("folder.sapic");

        assert!(create_request_group_result.is_ok());
        assert!(expected_path.exists());
        assert!(expected_spec_path.exists());

        // TODO: test the CreateRequestGroupOutput once implemented
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

    let create_request_group_result = collection.create_request_group(CreateRequestGroupInput {
        path: PathBuf::from(&request_group_name).join("inner"),
    }).await;
    assert!(create_request_group_result.is_ok());

    // Check creating the nested request group folder and its sapic spec file
    // FIXME: Should we create a spec file for all the folders created in the process
    // Or only the innermost one like we are doing now?

    let expected_path = collection_path.join(
        request_group_relative_path(&Path::new(&request_group_name).join("inner")),
    );
    let expected_spec_path = expected_path.join("folder.sapic");
    assert!(expected_path.exists());
    assert!(expected_spec_path.exists());

    // TODO: test the CreateRequestGroupOutput once implemented

    // Clean up
    {
        tokio::fs::remove_dir_all(&collection_path).await.unwrap()
    }

}