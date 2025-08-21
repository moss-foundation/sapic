#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::{random_collection_name, random_environment_name};
use moss_workspace::models::{
    operations::{CreateCollectionInput, CreateEnvironmentInput, UpdateEnvironmentGroupInput},
    types::{CreateCollectionParams, UpdateEnvironmentGroupParams},
};
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

pub mod shared;

#[tokio::test]
async fn update_environment_group_expand() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let collection_id = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                inner: CreateCollectionParams {
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
            CreateEnvironmentInput {
                collection_id: Some(collection_id.clone()),
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
                    collection_id: collection_id.clone(),
                    expanded: Some(false),
                    order: None,
                },
            },
        )
        .await
        .unwrap();

    let channel = Channel::new(move |_| Ok(()));

    let output = workspace
        .stream_environments(&ctx, channel.clone())
        .await
        .unwrap();

    assert!(!output.groups[0].expanded);

    // Setting the group back to expanded
    workspace
        .update_environment_group(
            &ctx,
            UpdateEnvironmentGroupInput {
                inner: UpdateEnvironmentGroupParams {
                    collection_id: collection_id.clone(),
                    expanded: Some(true),
                    order: None,
                },
            },
        )
        .await
        .unwrap();

    let output = workspace.stream_environments(&ctx, channel).await.unwrap();
    assert!(output.groups[0].expanded);

    cleanup().await;
}

#[tokio::test]
async fn update_environment_group_order() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;
    let collection_name = random_collection_name();
    let collection_id = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                inner: CreateCollectionParams {
                    name: collection_name.clone(),
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
            CreateEnvironmentInput {
                collection_id: Some(collection_id.clone()),
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
                    collection_id,
                    expanded: None,
                    order: Some(42),
                },
            },
        )
        .await
        .unwrap();

    let channel = Channel::new(move |_| Ok(()));

    let output = workspace.stream_environments(&ctx, channel).await.unwrap();

    assert_eq!(output.groups[0].order, Some(42));

    cleanup().await;
}
