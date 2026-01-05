#![cfg(feature = "integration-tests")]

use crate::shared::{set_up_test_main_window, test_stream_projects};
use moss_testutils::random_name::random_project_name;
use sapic_ipc::contracts::main::project::{
    CreateProjectInput, CreateProjectParams, UpdateProjectInput, UpdateProjectParams,
};

pub mod shared;

#[tokio::test]
async fn rename_project_nochange() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let old_project_name = random_project_name();
    let id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: old_project_name.to_string(),
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

    // No change in the update params
    main_window
        .update_project(
            &ctx,
            &UpdateProjectInput {
                inner: UpdateProjectParams {
                    id: id.clone(),
                    name: None,
                    icon_path: None,
                    order: None,
                    expanded: None,
                },
            },
        )
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;

    assert_eq!(projects[0].name, old_project_name);

    // Use the same name in update params
    main_window
        .update_project(
            &ctx,
            &UpdateProjectInput {
                inner: UpdateProjectParams {
                    id: id.clone(),
                    name: Some(old_project_name.to_string()),
                    icon_path: None,
                    order: None,
                    expanded: None,
                },
            },
        )
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert_eq!(projects[0].name, old_project_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_project_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let old_project_name = random_project_name();
    let id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: old_project_name.to_string(),
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

    let new_project_name = random_project_name();
    main_window
        .update_project(
            &ctx,
            &UpdateProjectInput {
                inner: UpdateProjectParams {
                    id,
                    name: Some(new_project_name.clone()),
                    icon_path: None,
                    order: None,
                    expanded: None,
                },
            },
        )
        .await
        .unwrap();

    let (_, projects) = test_stream_projects(&main_window, &ctx).await;
    assert_eq!(projects[0].name, new_project_name);

    cleanup().await;
}
