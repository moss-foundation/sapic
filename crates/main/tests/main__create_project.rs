#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_project_name;
use sapic_ipc::contracts::main::project::{CreateProjectInput, CreateProjectParams};

use crate::shared::{set_up_test_main_window, test_stream_projects};

mod shared;

// Many old tests no longer make sense:
// Special chars: We don't use project names for file name anymore
// Order and expanded: Those will be managed by the frontend

#[tokio::test]
async fn create_project_success() {
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

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].id, id);
    assert_eq!(projects[0].name, project_name);
    cleanup().await;
}

#[tokio::test]
async fn create_project_external_success() {
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

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].id, id);
    assert_eq!(projects[0].name, project_name);

    cleanup().await;
}
