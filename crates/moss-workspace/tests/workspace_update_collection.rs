mod shared;

use moss_common::api::OperationError;
use moss_testutils::random_name::random_collection_name;
use moss_workspace::models::operations::{CreateCollectionInput, UpdateCollectionEntryInput};

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn rename_collection_success() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: old_collection_name.clone(),
            order: None,
            external_path: None,
        })
        .await
        .unwrap();

    let new_collection_name = random_collection_name();
    let result = workspace
        .update_collection(UpdateCollectionEntryInput {
            id: create_collection_output.id,
            new_name: Some(new_collection_name.clone()),
        })
        .await;

    assert!(result.is_ok());

    cleanup().await;
}

#[tokio::test]
async fn rename_collection_empty_name() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: old_collection_name.clone(),
            order: None,
            external_path: None,
        })
        .await
        .unwrap();

    let new_collection_name = "".to_string();
    let rename_collection_result = workspace
        .update_collection(UpdateCollectionEntryInput {
            id: create_collection_output.id,
            new_name: Some(new_collection_name.clone()),
        })
        .await;

    assert!(matches!(
        rename_collection_result,
        Err(OperationError::InvalidInput(_))
    ));

    cleanup().await;
}

#[tokio::test]
async fn rename_collection_unchanged() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: old_collection_name.clone(),
            order: None,
            external_path: None,
        })
        .await
        .unwrap();

    let new_collection_name = old_collection_name;
    let rename_collection_result = workspace
        .update_collection(UpdateCollectionEntryInput {
            id: create_collection_output.id,
            new_name: Some(new_collection_name),
        })
        .await;

    assert!(rename_collection_result.is_ok());

    cleanup().await;
}

#[tokio::test]
async fn rename_collection_nonexistent_id() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    // Use a random ID that doesn't exist
    let nonexistent_id = uuid::Uuid::new_v4();

    let result = workspace
        .update_collection(UpdateCollectionEntryInput {
            id: nonexistent_id,
            new_name: Some(random_collection_name()),
        })
        .await;

    assert!(matches!(result, Err(OperationError::NotFound { .. })));

    cleanup().await;
}
