#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_project_name;
use sapic_ipc::contracts::main::project::{CreateProjectInput, CreateProjectParams};

use crate::shared::{set_up_test_main_window, test_stream_projects};

mod shared;
#[tokio::test]
async fn stream_projects_empty() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;

    assert_eq!(projects.len(), 0);
    cleanup().await;
}

#[tokio::test]
async fn stream_projects_one() {
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

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].id, id);
    assert_eq!(projects[0].name, project_name);

    cleanup().await;
}

#[tokio::test]
async fn stream_projects_multiple() {
    // Create one internal and one external project

    let (main_window, _delegate, ctx, cleanup, test_path) = set_up_test_main_window().await;

    let internal_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "Internal".to_string(),
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

    let external_path = test_path.join("External");

    let external_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "External".to_string(),
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

    assert_eq!(projects.len(), 2);
    assert!(
        projects
            .iter()
            .any(|p| p.id == internal_id && p.name == "Internal")
    );
    assert!(
        projects
            .iter()
            .any(|p| p.id == external_id && p.name == "External")
    );

    cleanup().await;
}
