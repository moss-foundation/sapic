#![cfg(feature = "integration-tests")]

// These tests should be done manually
// Since it requires authentication and env variables

use moss_storage::storage::operations::GetItem;
use moss_workspace::{
    models::{operations::CloneCollectionInput, primitives::CollectionId},
    storage::segments::{SEGKEY_COLLECTION, SEGKEY_EXPANDED_ITEMS},
};
use std::env;
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

pub mod shared;

#[tokio::test]
async fn clone_collection_success() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    dotenv::dotenv().ok();

    let clone_collection_output = workspace
        .clone_collection(
            &ctx,
            &CloneCollectionInput {
                order: 0,
                repository: env::var("GITHUB_COLLECTION_REPO_HTTPS").unwrap(),
            },
        )
        .await
        .unwrap();

    // Verify through stream_collections
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_collections(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the directory was created
    assert!(clone_collection_output.abs_path.exists());

    // Verify the db entries were created
    let id = clone_collection_output.id;
    let item_store = workspace.db().item_store();

    // Check order was stored
    let order_key = SEGKEY_COLLECTION.join(&id.to_string()).join("order");
    let order_value = GetItem::get(item_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: usize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 0);

    // Check expanded_items contains the collection id
    let expanded_items_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
    )
    .await
    .unwrap();
    let expanded_items: Vec<CollectionId> = expanded_items_value.deserialize().unwrap();
    assert!(expanded_items.contains(&id));

    // cleanup().await;
}
