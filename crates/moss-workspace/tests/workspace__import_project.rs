#![cfg(feature = "integration-tests")]

// TODO: Make this test work in CI

// These tests should be done manually
// Since it requires authentication and env variables

use crate::shared::{setup_external_project, setup_test_workspace};
use moss_project::models::primitives::ProjectId;
use moss_storage2::{Storage, models::primitives::StorageScope};
use moss_user::models::primitives::AccountId;
use moss_workspace::{
    models::{
        operations::ImportProjectInput,
        types::{ImportDiskParams, ImportGitHubParams, ImportProjectParams, ImportProjectSource},
    },
    storage::{KEY_EXPANDED_ITEMS, key_project_order},
};
use sapic_core::context::AnyAsyncContext;
use std::{collections::HashSet, env, ops::Deref};
use tauri::ipc::Channel;

pub mod shared;

#[ignore]
#[tokio::test]
async fn clone_project_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    dotenvy::dotenv().ok();

    let account_id = ctx
        .value("account_id")
        .unwrap()
        .downcast::<AccountId>()
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

    let id = clone_project_output.id;

    // Verify the db entries were created
    let storage = <dyn Storage>::global(&app_delegate);
    // Check order was stored
    let order_value = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            &key_project_order(&id),
        )
        .await
        .unwrap()
        .unwrap();
    let order: isize = serde_json::from_value(order_value).unwrap();

    assert_eq!(order, 0);
    // Check expanded_items contains the project id
    let expanded_items_value = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            KEY_EXPANDED_ITEMS,
        )
        .await
        .unwrap()
        .unwrap();
    let expanded_items: HashSet<ProjectId> = serde_json::from_value(expanded_items_value).unwrap();
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

    let id = import_project_output.id;

    // Verify the db entries were created
    let storage = <dyn Storage>::global(&app_delegate);
    // Check order was stored
    let order_value = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            &key_project_order(&id),
        )
        .await
        .unwrap()
        .unwrap();
    let order: isize = serde_json::from_value(order_value).unwrap();

    assert_eq!(order, 0);
    // Check expanded_items contains the project id
    let expanded_items_value = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            KEY_EXPANDED_ITEMS,
        )
        .await
        .unwrap()
        .unwrap();
    let expanded_items: HashSet<ProjectId> = serde_json::from_value(expanded_items_value).unwrap();
    assert!(expanded_items.contains(&id));

    tokio::fs::remove_dir_all(&external_path).await.unwrap();

    cleanup().await;
}
