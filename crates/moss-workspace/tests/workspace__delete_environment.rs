// TODO: Update the tests after changing the variable store to the new database
#![cfg(feature = "integration-tests")]

use moss_environment::{
    AnyEnvironment,
    models::{
        primitives::EnvironmentId,
        types::{AddVariableParams, VariableOptions},
    },
    storage::key_variable,
};
use moss_storage::{primitives::segkey::SegKeyBuf, storage::operations::GetItem};
use moss_storage2::{Storage, models::primitives::StorageScope};
use moss_testutils::random_name::random_environment_name;
use moss_workspace::{
    models::{
        operations::{CreateEnvironmentInput, DeleteEnvironmentInput, UpdateEnvironmentInput},
        types::UpdateEnvironmentParams,
    },
    storage::key_environment_order,
};
use serde_json::Value as JsonValue;
use tauri::ipc::Channel;

use crate::shared::setup_test_workspace;

mod shared;
#[tokio::test]
async fn delete_environment_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    // Create a custom environment with a variable
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
    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 1); // Only the globals environment should exist

    // Check the environment file is deleted
    assert!(!create_environment_output.abs_path.exists());

    // Check the environment is removed from the database
    let storage = <dyn Storage>::global(&app_delegate);

    let env_order_result = storage
        .get(
            StorageScope::Workspace(workspace.id().inner()),
            &key_environment_order(&environment_id),
        )
        .await
        .unwrap();
    assert!(env_order_result.is_none());

    // Check variables associated with the environment are removed from the database
    let storage = <dyn Storage>::global(&app_delegate);
    assert!(
        storage
            .get_batch_by_prefix(
                StorageScope::Workspace(workspace.id().inner()),
                &key_variable(&variable_id)
            )
            .await
            .unwrap()
            .is_empty()
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
