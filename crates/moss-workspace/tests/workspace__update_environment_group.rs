#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::{random_environment_name, random_project_name};
use moss_workspace::models::{
    operations::{CreateEnvironmentInput, CreateProjectInput, UpdateEnvironmentGroupInput},
    types::{CreateProjectParams, UpdateEnvironmentGroupParams},
};
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

pub mod shared;

#[tokio::test]
async fn update_environment_group_expand() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_project_name();
    let collection_id = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: collection_name,
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

    let environment_name = random_environment_name();
    let _ = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                project_id: Some(collection_id.clone()),
                name: environment_name.clone(),
                order: 0,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    // By default, newly created environment groups are expanded
    // We test with collapse it first and then expand

    workspace
        .update_environment_group(
            &ctx,
            UpdateEnvironmentGroupInput {
                inner: UpdateEnvironmentGroupParams {
                    project_id: collection_id.clone(),
                    expanded: Some(false),
                    order: None,
                },
            },
        )
        .await
        .unwrap();

    let channel = Channel::new(move |_| Ok(()));

    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel.clone())
        .await
        .unwrap();

    assert!(!output.groups[0].expanded);

    // Setting the group back to expanded
    workspace
        .update_environment_group(
            &ctx,
            UpdateEnvironmentGroupInput {
                inner: UpdateEnvironmentGroupParams {
                    project_id: collection_id.clone(),
                    expanded: Some(true),
                    order: None,
                },
            },
        )
        .await
        .unwrap();

    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();
    assert!(output.groups[0].expanded);

    cleanup().await;
}

#[tokio::test]
async fn update_environment_group_order() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;
    let project_name = random_project_name();
    let project_id = workspace
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

    let environment_name = random_environment_name();
    let _ = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                project_id: Some(project_id.clone()),
                name: environment_name,
                order: 0,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    workspace
        .update_environment_group(
            &ctx,
            UpdateEnvironmentGroupInput {
                inner: UpdateEnvironmentGroupParams {
                    project_id,
                    expanded: None,
                    order: Some(42),
                },
            },
        )
        .await
        .unwrap();

    let channel = Channel::new(move |_| Ok(()));

    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();

    assert_eq!(output.groups[0].order, Some(42));

    cleanup().await;
}
