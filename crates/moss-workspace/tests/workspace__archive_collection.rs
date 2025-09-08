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
pub async fn archive_collection_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let id = workspace
        .create_collection(
            &ctx,
            &app_delegate,
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

    // Check that the collection is initially not archived
    let (events, _stream_output) = test_stream_collections(&ctx, &workspace).await;
    assert!(!events.get(&id).unwrap().archived);

    workspace
        .archive_collection(&ctx, ArchiveCollectionInput { id: id.clone() })
        .await
        .unwrap();

    // Check that collection is flagged as archived during streaming
    let (events, _stream_output) = test_stream_collections(&ctx, &workspace).await;

    assert_eq!(events.len(), 1);
    assert!(events.get(&id).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn archive_collection_already_archived() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let id = workspace
        .create_collection(
            &ctx,
            &app_delegate,
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

    workspace
        .archive_collection(&ctx, ArchiveCollectionInput { id: id.clone() })
        .await
        .unwrap();

    let result = workspace
        .archive_collection(&ctx, ArchiveCollectionInput { id: id.clone() })
        .await;
    assert!(result.is_ok());

    // Check that collection is still flagged as archived during streaming
    let (events, _stream_output) = test_stream_collections(&ctx, &workspace).await;

    assert_eq!(events.len(), 1);
    assert!(events.get(&id).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn archived_collection_nonexistent() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

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
