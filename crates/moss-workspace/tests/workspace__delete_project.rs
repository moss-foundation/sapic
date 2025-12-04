#![cfg(feature = "integration-tests")]
pub mod shared;

use crate::shared::setup_test_workspace;
use moss_applib::mock::MockAppRuntime;
use moss_storage2::models::primitives::StorageScope;
use moss_testutils::random_name::random_project_name;
use moss_workspace::{
    models::{
        operations::{CreateProjectInput, DeleteProjectInput},
        types::CreateProjectParams,
    },
    storage::{KEY_EXPANDED_ITEMS, key_project},
};
use sapic_base::project::types::primitives::ProjectId;
use sapic_runtime::globals::GlobalKvStorage;
use std::collections::HashSet;
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
        .delete_project::<MockAppRuntime>(&ctx, &DeleteProjectInput { id: id.clone() })
        .await
        .unwrap();

    // Check updating projects
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_projects::<MockAppRuntime>(&ctx, channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 0);

    // Check updating database
    let storage = GlobalKvStorage::get(&app_delegate);

    // Check that project-specific entries are removed
    let project_prefix = key_project(&id);
    let list_result = storage
        .get_batch_by_prefix(
            StorageScope::Workspace(workspace.id().inner()),
            &project_prefix,
        )
        .await
        .unwrap();
    assert!(list_result.is_empty());

    // Check that expanded_items no longer contains the deleted project
    let expanded_items_value = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            KEY_EXPANDED_ITEMS,
        )
        .await
        .unwrap()
        .unwrap();
    let expanded_items: HashSet<ProjectId> = serde_json::from_value(expanded_items_value).unwrap();
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
        .delete_project::<MockAppRuntime>(&ctx, &DeleteProjectInput { id: id.clone() })
        .await
        .unwrap();

    // Delete the project again - should succeed but return None abs_path
    let delete_project_result = workspace
        .delete_project::<MockAppRuntime>(&ctx, &DeleteProjectInput { id: id.clone() })
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
        .delete_project::<MockAppRuntime>(
            &ctx,
            &DeleteProjectInput {
                id: create_project_output.id,
            },
        )
        .await
        .unwrap();

    // Check projects are updated
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_projects::<MockAppRuntime>(&ctx, channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 0);

    // TODO: Check database after implementing self-healing mechanism?

    cleanup().await;
}
