pub mod shared;

use moss_common::api::OperationError;
use moss_testutils::random_name::random_collection_name;
use moss_workspace::models::operations::{CreateCollectionInput, UpdateCollectionInput};

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn rename_collection_success() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: old_collection_name.clone(),
                order: None,
                external_path: None,
                repo: None,
            },
        )
        .await
        .unwrap();

    let new_collection_name = random_collection_name();
    let result = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: create_collection_output.id,
                new_name: Some(new_collection_name.clone()),
                new_repo: None,
                order: None,
                pinned: None,
            },
        )
        .await;

    assert!(result.is_ok());

    // Verify the manifest is updated
    let collections = workspace.collections(&ctx).await.unwrap();
    let collection = collections.iter().next().unwrap().1.read().await;
    assert_eq!(collection.manifest().await.name, new_collection_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_collection_empty_name() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: old_collection_name.clone(),
                order: None,
                external_path: None,
                repo: None,
            },
        )
        .await
        .unwrap();

    let new_collection_name = "".to_string();
    let rename_collection_result = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: create_collection_output.id,
                new_name: Some(new_collection_name.clone()),
                new_repo: None,
                order: None,
                pinned: None,
            },
        )
        .await;

    assert!(matches!(
        rename_collection_result,
        Err(OperationError::InvalidInput(_))
    ));

    cleanup().await;
}

#[tokio::test]
async fn rename_collection_unchanged() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: old_collection_name.clone(),
                order: None,
                external_path: None,
                repo: None,
            },
        )
        .await
        .unwrap();

    let new_collection_name = old_collection_name;
    let rename_collection_result = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: create_collection_output.id,
                new_name: Some(new_collection_name),
                new_repo: None,
                order: None,
                pinned: None,
            },
        )
        .await;

    assert!(rename_collection_result.is_ok());

    cleanup().await;
}

#[tokio::test]
async fn rename_collection_nonexistent_id() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    // Use a random ID that doesn't exist
    let nonexistent_id = uuid::Uuid::new_v4();

    let result = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: nonexistent_id,
                new_name: Some(random_collection_name()),
                new_repo: None,
                order: None,
                pinned: None,
            },
        )
        .await;

    assert!(matches!(result, Err(OperationError::NotFound { .. })));

    cleanup().await;
}

#[tokio::test]
async fn update_collection_repo() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let old_repo = "https://github.com/xxx/1.git";
    let new_repo = "https://github.com/xxx/2.git";
    let create_collection_output = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name,
                order: None,
                external_path: None,
                repo: Some(old_repo.to_string()),
            },
        )
        .await
        .unwrap();

    let result = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: create_collection_output.id,
                new_name: None,
                new_repo: Some(new_repo.to_string()),
                order: None,
                pinned: None,
            },
        )
        .await;

    assert!(result.is_ok());

    // Verify the manifest is updated
    let collections = workspace.collections(&ctx).await.unwrap();
    let collection = collections.iter().next().unwrap().1.read().await;

    assert_eq!(collection.manifest().await.repo, Some(new_repo.to_string()));

    cleanup().await;
}
