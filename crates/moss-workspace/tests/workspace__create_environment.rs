#![cfg(feature = "integration-tests")]

use moss_environment::{
    AnyEnvironment,
    models::types::{AddVariableParams, VariableOptions},
};
use moss_storage::storage::operations::GetItem;
use moss_testutils::random_name::{random_collection_name, random_environment_name};
use moss_workspace::{
    models::{
        operations::{CreateCollectionInput, CreateEnvironmentInput},
        types::CreateCollectionParams,
    },
    storage::segments::SEGKEY_ENVIRONMENT,
};
use serde_json::Value as JsonValue;
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

pub mod shared;

#[tokio::test]
async fn create_environment_success() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                collection_id: None,
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
    let output = workspace.stream_environments(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 2); // Expected two because of 1 global + 1 created

    assert!(create_environment_output.abs_path.exists());

    // Check the newly created environment is stored in the db
    let item_store = workspace.db().item_store();

    let stored_env_order: isize = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_ENVIRONMENT.join(id.as_str()).join("order"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(stored_env_order, 42);

    let env = workspace.environment(&id).await.unwrap();
    let variables = env.describe(&ctx).await.unwrap().variables;

    assert_eq!(variables.len(), 1);

    cleanup().await;
}

#[tokio::test]
async fn create_environment_already_exists() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    let environment_name = random_environment_name();
    let _ = workspace
        .create_environment(
            &ctx,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                collection_id: None,
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
            CreateEnvironmentInput {
                name: environment_name.clone(),
                collection_id: None,
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
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

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

    let create_environment_output = workspace
        .create_environment(
            &ctx,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                collection_id: Some(collection_id),
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
    let output = workspace.stream_environments(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 2); // Expected two because of 1 global + 1 created

    assert!(create_environment_output.abs_path.exists());

    // Check the newly created environment is stored in the db
    let item_store = workspace.db().item_store();

    let stored_env_order: isize = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_ENVIRONMENT.join(id.as_str()).join("order"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(stored_env_order, 42);

    let env = workspace.environment(&id).await.unwrap();
    let variables = env.describe(&ctx).await.unwrap().variables;

    assert_eq!(variables.len(), 1);

    cleanup().await;
}

#[tokio::test]
async fn create_collection_environment_already_exists() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

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
                name: environment_name.clone(),
                collection_id: Some(collection_id.clone()),
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
            CreateEnvironmentInput {
                name: environment_name.clone(),
                collection_id: Some(collection_id.clone()),
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
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

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

    let collection_environment_output = workspace
        .create_environment(
            &ctx,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                collection_id: Some(collection_id.clone()),
                order: 42,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let collection_env_id = collection_environment_output.id;

    let workspace_environment_output = workspace
        .create_environment(
            &ctx,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                collection_id: None,
                order: 42,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let workspace_env_id = workspace_environment_output.id;

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_environments(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 3); // 1 global + 1 workspace + 1 collection

    assert!(collection_environment_output.abs_path.exists());
    assert!(workspace_environment_output.abs_path.exists());

    // Check the newly created environment is stored in the db
    let item_store = workspace.db().item_store();

    let stored_collection_env_order: isize = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_ENVIRONMENT
            .join(collection_env_id.as_str())
            .join("order"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();

    assert_eq!(stored_collection_env_order, 42);

    let stored_workspace_env_order: isize = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_ENVIRONMENT
            .join(workspace_env_id.as_str())
            .join("order"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();

    assert_eq!(stored_workspace_env_order, 42);

    let _collection_env = workspace.environment(&collection_env_id).await.unwrap();
    let _workspace_env = workspace.environment(&workspace_env_id).await.unwrap();

    cleanup().await;
}
