#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_project_name;
use sapic_base::project::types::primitives::ProjectId;
use sapic_ipc::contracts::main::project::{
    ArchiveProjectInput, CreateProjectInput, CreateProjectParams, UnarchiveProjectInput,
};

use crate::shared::{set_up_test_main_window, test_stream_projects};

mod shared;

#[tokio::test]
pub async fn unarchive_project_success() {
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

    // Archive the project
    main_window
        .archive_project(&ctx, ArchiveProjectInput { id: id.clone() })
        .await
        .unwrap();

    // Unarchive the project
    main_window
        .unarchive_project(&ctx, UnarchiveProjectInput { id: id.clone() })
        .await
        .unwrap();

    // Check that the project is not archived
    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert!(!projects.get(0).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn unarchive_project_already_unarchived() {
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

    // Unarchive the project, which by default is unarchived
    main_window
        .unarchive_project(&ctx, UnarchiveProjectInput { id: id.clone() })
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert!(!projects.get(0).unwrap().archived);

    cleanup().await;
}

#[tokio::test]
pub async fn unarchive_project_nonexistent() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let id = ProjectId::new();

    let result = main_window
        .unarchive_project(&ctx, UnarchiveProjectInput { id })
        .await;

    assert!(result.is_err());

    cleanup().await;
}
