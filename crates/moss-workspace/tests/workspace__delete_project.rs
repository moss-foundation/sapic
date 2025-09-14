#![cfg(feature = "integration-tests")]
pub mod shared;

use crate::shared::setup_test_workspace;
use moss_storage::storage::operations::{GetItem, ListByPrefix};
use moss_testutils::random_name::random_project_name;
use moss_workspace::{
    models::{
        operations::{CreateProjectInput, DeleteProjectInput},
        primitives::ProjectId,
        types::CreateProjectParams,
    },
    storage::segments::{SEGKEY_COLLECTION, SEGKEY_EXPANDED_ITEMS},
};
use tauri::ipc::Channel;

#[tokio::test]
async fn delete_project_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = random_project_name();
    let create_project_output = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    let id = create_project_output.id;
    let _ = workspace
        .delete_project(&ctx, &DeleteProjectInput { id: id.clone() })
        .await
        .unwrap();

    // Check updating projects
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_projects(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 0);

    // Check updating database - project metadata should be removed
    let item_store = workspace.db().item_store();

    // Check that project-specific entries are removed
    let project_prefix = SEGKEY_COLLECTION.join(&id.to_string());
    let list_result =
        ListByPrefix::list_by_prefix(item_store.as_ref(), &ctx, &project_prefix.to_string())
            .await
            .unwrap();
    assert!(list_result.is_empty());

    // Check that expanded_items no longer contains the deleted project
    let expanded_items_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
    )
    .await
    .unwrap();
    let expanded_items: Vec<ProjectId> = expanded_items_value.deserialize().unwrap();
    assert!(!expanded_items.contains(&id));

    cleanup().await;
}

#[tokio::test]
async fn delete_project_nonexistent_id() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = random_project_name();
    let id = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
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
        .delete_project(&ctx, &DeleteProjectInput { id: id.clone() })
        .await
        .unwrap();

    // Delete the project again - should succeed but return None abs_path
    let delete_project_result = workspace
        .delete_project(&ctx, &DeleteProjectInput { id: id.clone() })
        .await
        .unwrap();

    // The second deletion should succeed but indicate nothing was deleted
    assert!(delete_project_result.abs_path.is_none());

    cleanup().await;
}

#[tokio::test]
async fn delete_project_fs_already_deleted() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = random_project_name();
    let create_project_output = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    // Delete the project manually from the filesystem
    tokio::fs::remove_dir_all(&*create_project_output.abs_path)
        .await
        .unwrap();

    // Even though filesystem is already deleted, deletion should succeed
    let _ = workspace
        .delete_project(
            &ctx,
            &DeleteProjectInput {
                id: create_project_output.id,
            },
        )
        .await
        .unwrap();

    // Check projects are updated
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_projects(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 0);

    // TODO: Check database after implementing self-healing mechanism?

    cleanup().await;
}
