#![cfg(feature = "integration-tests")]

use moss_project::models::primitives::ProjectId;
use moss_testutils::random_name::random_project_name;
use moss_workspace::models::{
    operations::{ArchiveProjectInput, CreateProjectInput},
    types::CreateProjectParams,
};

use crate::shared::{setup_test_workspace, test_stream_projects};

pub mod shared;

#[tokio::test]
pub async fn archive_project_success() {
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

    // Check that the project is initially not archived
    let (events, _stream_output) = test_stream_projects(&ctx, &workspace).await;
    assert!(!events.get(&id).unwrap().archived);

    workspace
        .archive_project(&ctx, ArchiveProjectInput { id: id.clone() })
        .await
        .unwrap();

    // Check that project is flagged as archived during streaming
    let (events, _stream_output) = test_stream_projects(&ctx, &workspace).await;

    assert_eq!(events.len(), 1);
    assert!(events.get(&id).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn archive_project_already_archived() {
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
        .archive_project(&ctx, ArchiveProjectInput { id: id.clone() })
        .await
        .unwrap();

    let result = workspace
        .archive_project(&ctx, ArchiveProjectInput { id: id.clone() })
        .await;
    assert!(result.is_ok());

    // Check that project is still flagged as archived during streaming
    let (events, _stream_output) = test_stream_projects(&ctx, &workspace).await;

    assert_eq!(events.len(), 1);
    assert!(events.get(&id).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn archived_project_nonexistent() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    let result = workspace
        .archive_project(
            &ctx,
            ArchiveProjectInput {
                id: ProjectId::new(),
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}
