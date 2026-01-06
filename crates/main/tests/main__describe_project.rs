#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_project_name;
use sapic_base::project::types::primitives::ProjectId;
use sapic_ipc::contracts::main::project::{
    CreateProjectInput, CreateProjectParams, DescribeProjectInput,
};

use crate::shared::set_up_test_main_window;

mod shared;

#[tokio::test]
async fn describe_project_internal() {
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

    let description = main_window
        .describe_project(&ctx, &DescribeProjectInput { id })
        .await
        .unwrap();

    assert_eq!(description.name, project_name);

    cleanup().await;
}

#[tokio::test]
async fn describe_project_external() {
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

    let description = main_window
        .describe_project(&ctx, &DescribeProjectInput { id })
        .await
        .unwrap();

    assert_eq!(description.name, project_name);

    cleanup().await;
}

#[tokio::test]
async fn describe_project_nonexistent() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let result = main_window
        .describe_project(
            &ctx,
            &DescribeProjectInput {
                id: ProjectId::new(),
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}
