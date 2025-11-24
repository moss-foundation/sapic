#![cfg(feature = "integration-tests")]

use moss_environment::{
    AnyEnvironment,
    models::types::{AddVariableParams, VariableOptions},
};
use moss_storage2::{Storage, models::primitives::StorageScope};
use moss_testutils::random_name::{random_environment_name, random_project_name};
use moss_workspace::{
    models::{
        operations::{CreateEnvironmentInput, CreateProjectInput},
        types::CreateProjectParams,
    },
    storage::key_environment_order,
};
use serde_json::Value as JsonValue;
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

pub mod shared;

#[tokio::test]
async fn create_environment_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: None,
                order: 42,
                color: Some("#3574F0".to_string()),
                variables: vec![AddVariableParams {
                    name: "TEST_VAR".to_string(),
                    global_value: JsonValue::String("test".to_string()),
                    local_value: JsonValue::String("test".to_string()),
                    order: 42,
                    desc: Some("test".to_string()),
                    options: VariableOptions { disabled: false },
                }],
            },
        )
        .await
        .unwrap();

    let id = create_environment_output.id;

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 2); // Expected two because of 1 global + 1 created

    assert!(create_environment_output.abs_path.exists());

    // Check the newly created environment is stored in the db
    let storage = <dyn Storage>::global(&app_delegate);
    let stored_env_order_value = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            &key_environment_order(&id),
        )
        .await
        .unwrap()
        .unwrap();
    let stored_env_order: isize = serde_json::from_value(stored_env_order_value).unwrap();
    assert_eq!(stored_env_order, 42);

    let env = workspace.environment(&id).await.unwrap();
    let variables = env.describe(&ctx).await.unwrap().variables;

    assert_eq!(variables.len(), 1);

    cleanup().await;
}

#[tokio::test]
async fn create_environment_already_exists() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let environment_name = random_environment_name();
    let _ = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: None,
                order: 42,
                color: Some("#3574F0".to_string()),
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let result = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: None,
                order: 42,
                color: Some("#3574F0".to_string()),
                variables: vec![],
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn create_collection_environment_success() {
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

    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: Some(collection_id),
                order: 42,
                color: None,
                variables: vec![AddVariableParams {
                    name: "TEST_VAR".to_string(),
                    global_value: JsonValue::String("test".to_string()),
                    local_value: JsonValue::String("test".to_string()),
                    order: 42,
                    desc: Some("test".to_string()),
                    options: VariableOptions { disabled: false },
                }],
            },
        )
        .await
        .unwrap();

    let id = create_environment_output.id;

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 2); // Expected two because of 1 global + 1 created

    assert!(create_environment_output.abs_path.exists());

    // Check the newly created environment is stored in the db
    let storage = <dyn Storage>::global(&app_delegate);
    let stored_env_order_value = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            &key_environment_order(&id),
        )
        .await
        .unwrap()
        .unwrap();
    let stored_env_order: isize = serde_json::from_value(stored_env_order_value).unwrap();
    assert_eq!(stored_env_order, 42);

    let env = workspace.environment(&id).await.unwrap();
    let variables = env.describe(&ctx).await.unwrap().variables;

    assert_eq!(variables.len(), 1);

    cleanup().await;
}

#[tokio::test]
async fn create_collection_environment_already_exists() {
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
                name: environment_name.clone(),
                project_id: Some(collection_id.clone()),
                order: 42,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let result = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: Some(collection_id.clone()),
                order: 42,
                color: None,
                variables: vec![],
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn create_collection_environment_same_name_as_workspace_environment() {
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

    let collection_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: Some(collection_id.clone()),
                order: 42,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let project_env_id = collection_environment_output.id;

    let workspace_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: None,
                order: 42,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let workspace_env_id = workspace_environment_output.id;

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 3); // 1 global + 1 workspace + 1 collection

    assert!(collection_environment_output.abs_path.exists());
    assert!(workspace_environment_output.abs_path.exists());

    // Check the newly created environment is stored in the db
    let storage = <dyn Storage>::global(&app_delegate);
    let stored_project_env_order_value = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            &key_environment_order(&project_env_id),
        )
        .await
        .unwrap()
        .unwrap();
    let stored_project_env_order: isize =
        serde_json::from_value(stored_project_env_order_value).unwrap();
    assert_eq!(stored_project_env_order, 42);

    let stored_workspace_env_order_value = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            &key_environment_order(&workspace_env_id),
        )
        .await
        .unwrap()
        .unwrap();
    let stored_workspace_env_order: isize =
        serde_json::from_value(stored_workspace_env_order_value).unwrap();
    assert_eq!(stored_workspace_env_order, 42);

    let _collection_env = workspace.environment(&project_env_id).await.unwrap();
    let _workspace_env = workspace.environment(&workspace_env_id).await.unwrap();

    cleanup().await;
}
