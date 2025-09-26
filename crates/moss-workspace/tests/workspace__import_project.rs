#![cfg(feature = "integration-tests")]

// TODO: Make this test work in CI

// These tests should be done manually
// Since it requires authentication and env variables

use moss_applib::context::AnyAsyncContext;
use moss_storage::storage::operations::GetItem;
use moss_user::models::primitives::AccountId;
use moss_workspace::{
    models::{
        operations::ImportProjectInput,
        primitives::ProjectId,
        types::{ImportDiskParams, ImportGitHubParams, ImportProjectParams, ImportProjectSource},
    },
    storage::segments::{SEGKEY_COLLECTION, SEGKEY_EXPANDED_ITEMS},
};
use std::{env, ops::Deref, path::Path};
use tauri::ipc::Channel;

use crate::shared::{setup_external_project, setup_test_workspace};

pub mod shared;

#[ignore]
#[tokio::test]
async fn clone_project_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    dotenv::dotenv().ok();

    let account_id = ctx
        .value::<AccountId>("account_id")
        .unwrap()
        .deref()
        .clone();

    let clone_project_output = workspace
        .import_project(
            &ctx,
            &app_delegate,
            &ImportProjectInput {
                inner: ImportProjectParams {
                    name: "New Project".to_string(),
                    order: 0,
                    icon_path: None,
                    source: ImportProjectSource::GitHub(ImportGitHubParams {
                        repository: env::var("GITHUB_PROJECT_REPO_HTTPS").unwrap(),
                        branch: None,
                        account_id,
                    }),
                },
            },
        )
        .await
        .unwrap();

    // Verify through stream_projects
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_projects(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the directory was created
    assert!(clone_project_output.abs_path.exists());

    // Verify the db entries were created
    let id = clone_project_output.id;
    let item_store = workspace.db().item_store();

    // Check order was stored
    let order_key = SEGKEY_COLLECTION.join(&id.to_string()).join("order");
    let order_value = GetItem::get(item_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: usize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 0);

    // Check expanded_items contains the project id
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

#[tokio::test]
async fn import_external_project_success() {
    // Create an external project and import it
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let (project_name, external_path) =
        setup_external_project(&ctx, &app_delegate, &workspace).await;

    // Import the external project
    let import_project_output = workspace
        .import_project(
            &ctx,
            &app_delegate,
            &ImportProjectInput {
                inner: ImportProjectParams {
                    name: project_name.clone(),
                    order: 0,
                    source: ImportProjectSource::Disk(ImportDiskParams {
                        external_path: external_path.clone(),
                    }),
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    // Verify through stream_projects
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_projects(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the internal directory was created
    assert!(import_project_output.abs_path.exists());

    // Verify the db entries were created
    let id = import_project_output.id;
    let item_store = workspace.db().item_store();

    // Check order was stored
    let order_key = SEGKEY_COLLECTION.join(&id.to_string()).join("order");
    let order_value = GetItem::get(item_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: usize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 0);

    // Check expanded_items contains the project id
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
