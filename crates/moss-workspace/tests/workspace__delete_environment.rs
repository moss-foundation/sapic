#![cfg(feature = "integration-tests")]

use moss_environment::{
    AnyEnvironment,
    models::{
        primitives::EnvironmentId,
        types::{AddVariableParams, VariableOptions},
    },
    segments::{SEGKEY_VARIABLE_LOCALVALUE, SEGKEY_VARIABLE_ORDER},
};
use moss_storage::{primitives::segkey::SegKeyBuf, storage::operations::GetItem};
use moss_testutils::random_name::random_environment_name;
use moss_workspace::{
    models::{
        operations::{CreateEnvironmentInput, DeleteEnvironmentInput, UpdateEnvironmentInput},
        types::UpdateEnvironmentParams,
    },
    storage::segments::SEGKEY_ENVIRONMENT,
};
use serde_json::Value as JsonValue;
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

mod shared;
#[tokio::test]
async fn delete_environment_success() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    // Create a custom environment with a variable
    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
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

    let environment_id = create_environment_output.id;

    let var = AddVariableParams {
        name: "1".to_string(),
        global_value: JsonValue::String("variable 1".to_string()),
        local_value: JsonValue::String("variable 1".to_string()),
        order: 1,
        desc: Some("First variable".to_string()),
        options: VariableOptions { disabled: true },
    };

    // Add a variable
    let _ = workspace
        .update_environment(
            &ctx,
            UpdateEnvironmentInput {
                inner: UpdateEnvironmentParams {
                    id: environment_id.clone(),
                    name: None,
                    order: None,
                    color: None,
                    expanded: None,
                    vars_to_add: vec![var],
                    vars_to_delete: vec![],
                    vars_to_update: vec![],
                },
            },
        )
        .await
        .unwrap();

    let env_description = workspace
        .environment(&environment_id)
        .await
        .unwrap()
        .describe(&ctx)
        .await
        .unwrap();

    let variables = env_description.variables;

    let variable_id = variables.iter().next().unwrap().0.clone();

    // Delete the environment
    let _ = workspace
        .delete_environment(
            &ctx,
            DeleteEnvironmentInput {
                id: environment_id.clone(),
            },
        )
        .await
        .unwrap();

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace.stream_environments(&ctx, channel).await.unwrap();
    assert_eq!(output.total_returned, 1); // Only the globals environment should exist

    // Check the environment file is deleted
    assert!(!create_environment_output.abs_path.exists());

    // Check the environment is removed from the database
    let item_store = workspace.db().item_store();

    assert!(
        GetItem::get(
            item_store.as_ref(),
            &ctx,
            SEGKEY_ENVIRONMENT
                .join(environment_id.as_str())
                .join("order")
        )
        .await
        .is_err()
    );

    // Check variables associated with the environment are removed from the database
    let variable_store = workspace.db().variable_store();

    let segkey_localvalue = SegKeyBuf::from(variable_id.as_str()).join(SEGKEY_VARIABLE_LOCALVALUE);

    assert!(
        GetItem::get(variable_store.as_ref(), &ctx, segkey_localvalue,)
            .await
            .is_err()
    );

    let segkey_order = SegKeyBuf::from(variable_id.as_str()).join(SEGKEY_VARIABLE_ORDER);

    assert!(
        GetItem::get(variable_store.as_ref(), &ctx, segkey_order,)
            .await
            .is_err()
    );

    cleanup().await;
}

#[tokio::test]
async fn delete_environment_nonexistent() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    let result = workspace
        .delete_environment(
            &ctx,
            DeleteEnvironmentInput {
                id: EnvironmentId::new(),
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}
