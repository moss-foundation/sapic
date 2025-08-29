#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_collection_name;
use moss_workspace::models::{
    operations::{ArchiveCollectionInput, CreateCollectionInput},
    primitives::CollectionId,
    types::CreateCollectionParams,
};

use crate::shared::{setup_test_workspace, test_stream_collections};

pub mod shared;

#[tokio::test]
async fn archive_collection_success() {
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

    let output = workspace
        .archive_collection(
            &ctx,
            ArchiveCollectionInput {
                id: collection_id.clone(),
            },
        )
        .await
        .unwrap();

    // Check that the collection folder still exists
    assert!(output.abs_path.exists());

    // Check that stream_collections shows it to be archived
    let (events, output) = test_stream_collections(&ctx, &workspace).await;

    assert_eq!(events.len(), 1);
    assert_eq!(output.total_returned, 1);

    assert!(events.get(&collection_id).unwrap().archived);

    // Check that archived collection's handle is dropped
    assert!(workspace.collection(&collection_id).await.is_none());

    cleanup().await;
}

#[tokio::test]
async fn archive_collection_nonexistent() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let result = workspace
        .archive_collection(
            &ctx,
            ArchiveCollectionInput {
                id: CollectionId::new(),
            },
        )
        .await;
    assert!(result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn archive_collection_already_archived() {
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
        .archive_collection(
            &ctx,
            ArchiveCollectionInput {
                id: collection_id.clone(),
            },
        )
        .await;

    assert!(result.is_ok());

    cleanup().await;
}
