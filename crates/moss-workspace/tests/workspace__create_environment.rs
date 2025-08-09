#![cfg(feature = "integration-tests")]

use moss_environment::{
    AnyEnvironment,
    models::{
        primitives::EnvironmentId,
        types::{AddVariableParams, VariableOptions},
    },
};
use moss_storage::storage::operations::GetItem;
use moss_testutils::random_name::random_environment_name;
use moss_workspace::{
    models::operations::CreateEnvironmentInput,
    storage::segments::{SEGKEY_ENVIRONMENT, SEGKEY_EXPANDED_ENVIRONMENTS},
};
use serde_json::Value as JsonValue;
use std::collections::HashSet;
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

pub mod shared;

#[tokio::test]
async fn create_environment_success() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

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

    let stored_expanded_environments: HashSet<EnvironmentId> = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_EXPANDED_ENVIRONMENTS.to_segkey_buf(),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();

    assert!(stored_expanded_environments.contains(&id));

    let env = workspace.environment(&id).await.unwrap();
    let variables = env.describe(&ctx).await.unwrap().variables;

    assert_eq!(variables.len(), 1);

    cleanup().await;
}

#[tokio::test]
async fn create_environment_already_exists() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

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
