#![cfg(feature = "integration-tests")]

use moss_storage::storage::operations::GetItem;
use moss_testutils::random_name::random_collection_name;
use moss_workspace::{
    models::{
        operations::{CreateCollectionInput, ExportCollectionInput, ImportCollectionInput},
        primitives::ProjectId,
        types::{
            ArchiveImportParams, CreateCollectionParams, ExportCollectionParams,
            ImportCollectionParams, ImportCollectionSource,
        },
    },
    storage::segments::{SEGKEY_COLLECTION, SEGKEY_EXPANDED_ITEMS},
};
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

mod shared;

#[tokio::test]
pub async fn export_collection_success() {
    // Create an archive file from a collection and import it back
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let destination = workspace.abs_path().to_path_buf();
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

    let archive_path = workspace
        .export_collection(
            &ctx,
            &ExportCollectionInput {
                inner: ExportCollectionParams { id, destination },
            },
        )
        .await
        .unwrap()
        .archive_path;

    assert!(archive_path.exists());

    // Import from the exported archive file
    let import_collection_output = workspace
        .import_collection(
            &ctx,
            &app_delegate,
            &ImportCollectionInput {
                inner: ImportCollectionParams {
                    name: collection_name.clone(),
                    order: 42,
                    external_path: None,
                    source: ImportCollectionSource::Archive(ArchiveImportParams { archive_path }),
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    // Check that the imported collection has the same name as the exported one
    assert_eq!(import_collection_output.name, collection_name);

    // Verify through stream_collections
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_collections(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 2); // 1 created + 1 imported

    // Verify the directory was created
    assert!(import_collection_output.abs_path.exists());

    // Verify the db entries were created
    // Verify the db entries were created
    let id = import_collection_output.id;
    let item_store = workspace.db().item_store();

    // Check order was stored
    let order_key = SEGKEY_COLLECTION.join(&id.to_string()).join("order");
    let order_value = GetItem::get(item_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: usize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 42);

    // Check expanded_items contains the collection id
    let expanded_items_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
    )
    .await
    .unwrap();
    let expanded_items: Vec<ProjectId> = expanded_items_value.deserialize().unwrap();
    assert!(expanded_items.contains(&id));

    cleanup().await;
}
