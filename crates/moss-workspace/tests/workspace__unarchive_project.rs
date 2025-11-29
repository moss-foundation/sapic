#![cfg(feature = "integration-tests")]

use moss_applib::mock::MockAppRuntime;
use moss_project::models::primitives::ProjectId;
use moss_testutils::random_name::random_project_name;
use moss_workspace::models::{
    operations::{ArchiveProjectInput, CreateProjectInput, UnarchiveProjectInput},
    types::CreateProjectParams,
};

use crate::shared::{setup_test_workspace, test_stream_projects};

pub mod shared;

#[tokio::test]
pub async fn unarchive_project_success() {
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

    // First archive the project and unarchive it
    workspace
        .archive_project::<MockAppRuntime>(&ctx, ArchiveProjectInput { id: id.clone() })
        .await
        .unwrap();

    workspace
        .unarchive_project::<MockAppRuntime>(&ctx, UnarchiveProjectInput { id: id.clone() })
        .await
        .unwrap();

    // Check that the project is now unarchived
    // Check that project is flagged as archived during streaming
    let (events, _stream_output) = test_stream_projects::<MockAppRuntime>(&ctx, &workspace).await;

    assert_eq!(events.len(), 1);
    assert!(!events.get(&id).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn unarchive_project_already_unarchived() {
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

    // Check that the project is already unarchived
    let (events, _stream_output) = test_stream_projects::<MockAppRuntime>(&ctx, &workspace).await;
    assert!(!events.get(&id).unwrap().archived);

    let result = workspace
        .unarchive_project::<MockAppRuntime>(&ctx, UnarchiveProjectInput { id: id.clone() })
        .await;
    assert!(result.is_ok());

    // Check that project is still unarchived
    let (events, _stream_output) = test_stream_projects::<MockAppRuntime>(&ctx, &workspace).await;

    assert_eq!(events.len(), 1);
    assert!(!events.get(&id).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn unarchived_project_nonexistent() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;
    let result = workspace
        .unarchive_project::<MockAppRuntime>(
            &ctx,
            UnarchiveProjectInput {
                id: ProjectId::new(),
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}
