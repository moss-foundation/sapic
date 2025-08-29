#![cfg(feature = "integration-tests")]

use crate::shared::{setup_test_workspace, test_stream_collections};
use moss_testutils::random_name::random_collection_name;
use moss_workspace::models::{
    operations::{ArchiveCollectionInput, CreateCollectionInput, UnarchiveCollectionInput},
    primitives::CollectionId,
    types::CreateCollectionParams,
};

pub mod shared;

#[tokio::test]
async fn unarchive_collection_success() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let collection_id = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                inner: CreateCollectionParams {
                    name: collection_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    let _ = workspace
        .archive_collection(
            &ctx,
            ArchiveCollectionInput {
                id: collection_id.clone(),
            },
        )
        .await
        .unwrap();

    let result = workspace
        .unarchive_collection(
            &ctx,
            UnarchiveCollectionInput {
                id: collection_id.clone(),
            },
        )
        .await;

    assert!(result.is_ok());

    // Check that stream_collections shows the collection as not archived
    let (events, output) = test_stream_collections(&ctx, &workspace).await;
    assert_eq!(events.len(), 1);
    assert_eq!(output.total_returned, 1);

    assert!(!events.get(&collection_id).unwrap().archived);

    // Check that the collection handle is recreated
    assert!(workspace.collection(&collection_id).await.is_some());

    cleanup().await;
}

#[tokio::test]
async fn unarchive_collection_nonexistent() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let result = workspace
        .unarchive_collection(
            &ctx,
            UnarchiveCollectionInput {
                id: CollectionId::new(),
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn unarchive_collection_already_unarchived() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;
    let collection_name = random_collection_name();
    let collection_id = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                inner: CreateCollectionParams {
                    name: collection_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    let result = workspace
        .unarchive_collection(
            &ctx,
            UnarchiveCollectionInput {
                id: collection_id.clone(),
            },
        )
        .await;

    assert!(result.is_ok());

    cleanup().await;
}
