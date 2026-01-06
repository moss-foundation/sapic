#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_project_name;
use sapic_base::project::types::primitives::ProjectId;
use sapic_ipc::contracts::main::project::{
    CreateProjectInput, CreateProjectParams, DeleteProjectInput,
};

use crate::shared::{set_up_test_main_window, test_stream_projects};

mod shared;

#[tokio::test]
async fn delete_project_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_name = random_project_name();
    let id = main_window
        .create_project(
            &ctx,
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

    main_window
        .delete_project(&ctx, &DeleteProjectInput { id })
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert_eq!(projects.len(), 0);
    cleanup().await;
}

// We should gracefully handle this scenario
#[tokio::test]
async fn delete_project_nonexistent() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let id = ProjectId::new();

    let result = main_window
        .delete_project(&ctx, &DeleteProjectInput { id })
        .await;

    assert!(result.is_ok());

    cleanup().await;
}

#[tokio::test]
async fn delete_project_external() {
    let (main_window, _delegate, ctx, cleanup, test_path) = set_up_test_main_window().await;

    let project_name = random_project_name();
    let external_path = test_path.join(&project_name);

    let id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.to_string(),
                    order: 0,
                    external_path: Some(external_path.clone()),
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    main_window
        .delete_project(&ctx, &DeleteProjectInput { id })
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert_eq!(projects.len(), 0);
    cleanup().await;
}
