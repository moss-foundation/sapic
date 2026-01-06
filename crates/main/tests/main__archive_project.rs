#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_project_name;
use sapic_base::project::types::primitives::ProjectId;
use sapic_ipc::contracts::main::project::{
    ArchiveProjectInput, CreateProjectInput, CreateProjectParams,
};

use crate::shared::{set_up_test_main_window, test_stream_projects};

mod shared;

#[tokio::test]
pub async fn archive_project_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_name = random_project_name();
    let id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.to_string(),
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
    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert!(!projects.get(0).unwrap().archived);

    main_window
        .archive_project(&ctx, ArchiveProjectInput { id })
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert!(projects.get(0).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn archive_project_already_archived() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_name = random_project_name();
    let id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.to_string(),
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

    main_window
        .archive_project(&ctx, ArchiveProjectInput { id: id.clone() })
        .await
        .unwrap();

    // Archive the same project twice
    main_window
        .archive_project(&ctx, ArchiveProjectInput { id: id.clone() })
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert!(projects.get(0).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn archive_project_nonexistent() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let id = ProjectId::new();

    let result = main_window
        .archive_project(&ctx, ArchiveProjectInput { id })
        .await;

    assert!(result.is_err());

    cleanup().await;
}
