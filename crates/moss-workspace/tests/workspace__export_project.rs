#![cfg(feature = "integration-tests")]

use crate::shared::setup_test_workspace;
use moss_project::models::primitives::ProjectId;
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use moss_testutils::random_name::random_project_name;
use moss_workspace::{
    models::{
        operations::{CreateProjectInput, ExportProjectInput, ImportProjectInput},
        types::{
            CreateProjectParams, ExportProjectParams, ImportArchiveParams, ImportProjectParams,
            ImportProjectSource,
        },
    },
    storage::{KEY_EXPANDED_ITEMS, key_project_order},
};
use std::collections::HashSet;
use tauri::ipc::Channel;

mod shared;

#[tokio::test]
pub async fn export_project_success() {
    // Create an archive file from a project and import it back
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let destination = workspace.abs_path().to_path_buf();
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

    let archive_path = workspace
        .export_project(
            &ctx,
            &ExportProjectInput {
                inner: ExportProjectParams { id, destination },
            },
        )
        .await
        .unwrap()
        .archive_path;

    assert!(archive_path.exists());

    // Import from the exported archive file
    let import_project_output = workspace
        .import_project(
            &ctx,
            &app_delegate,
            &ImportProjectInput {
                inner: ImportProjectParams {
                    name: project_name.clone(),
                    order: 42,
                    source: ImportProjectSource::Archive(ImportArchiveParams { archive_path }),
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    // Check that the imported project has the same name as the exported one
    assert_eq!(import_project_output.name, project_name);

    // Verify through stream_projects
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_projects(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 2); // 1 created + 1 imported

    // Verify the directory was created
    assert!(import_project_output.abs_path.exists());

    let id = import_project_output.id;

    // Verify the db entries were created
    let storage = <dyn KvStorage>::global(&app_delegate);

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

    assert_eq!(order, 42);
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
