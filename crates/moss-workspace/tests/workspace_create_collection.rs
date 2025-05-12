mod shared;

use moss_common::api::OperationError;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_collection_name};
use moss_workspace::models::operations::CreateCollectionInput;

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn create_collection_success() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_result = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await;

    assert!(create_collection_result.is_ok());

    let create_collection_output = create_collection_result.unwrap();
    let collections = workspace.collections().await.unwrap().read().await;

    assert_eq!(collections.len(), 1);
    assert_eq!(
        collections[&create_collection_output.id].display_name,
        collection_name
    );

    // Verify the directory was created
    assert!(create_collection_output.abs_path.exists());

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_collection_empty_name() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = "".to_string();
    let create_collection_result = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await;

    assert!(matches!(
        create_collection_result,
        Err(OperationError::Validation(_))
    ));

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_collection_already_exists() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await
        .unwrap();

    let create_collection_result = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await;

    assert!(matches!(
        create_collection_result,
        Err(OperationError::AlreadyExists { .. })
    ));

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn create_collection_special_chars() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let collection_name_list = FILENAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_collection_name()))
        .collect::<Vec<String>>();

    for collection_name in collection_name_list {
        let create_collection_result = workspace
            .create_collection(CreateCollectionInput {
                name: collection_name.clone(),
            })
            .await;
        assert!(create_collection_result.is_ok());

        let create_collection_output = create_collection_result.unwrap();
        let collections = workspace.collections().await.unwrap().read().await;

        assert!(collections.contains_key(&create_collection_output.id));
        assert_eq!(
            collections[&create_collection_output.id].display_name,
            collection_name
        );

        // Verify the directory was created
        assert!(create_collection_output.abs_path.exists());
    }

    // Clean up
    cleanup().await;
}
