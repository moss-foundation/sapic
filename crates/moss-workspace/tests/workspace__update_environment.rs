#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_bindingutils::primitives::ChangeString;
use moss_environment::{
    AnyEnvironment,
    models::types::{AddVariableParams, VariableOptions},
};
use moss_testutils::random_name::random_environment_name;
use moss_workspace::models::operations::{CreateEnvironmentInput, UpdateEnvironmentInput};
use serde_json::Value as JsonValue;

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn update_environment_success() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let old_environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            CreateEnvironmentInput {
                name: old_environment_name.clone(),
                collection_id: None,
                order: 0,
                color: Some("#ffffff".to_string()),
            },
        )
        .await
        .unwrap();

    let new_environment_name = random_environment_name();
    let _ = workspace
        .update_environment(
            &ctx,
            UpdateEnvironmentInput {
                id: create_environment_output.id.clone(),
                name: Some(new_environment_name.clone()),
                collection_id: None,
                order: None,
                color: Some(ChangeString::Update("#000000".to_string())),
                expanded: None,
                vars_to_add: vec![AddVariableParams {
                    name: "TEST_VAR".to_string(),
                    global_value: JsonValue::String("test".to_string()),
                    local_value: JsonValue::String("test".to_string()),
                    order: 0,
                    desc: None,
                    options: VariableOptions { disabled: false },
                }],
                vars_to_update: vec![],
                vars_to_delete: vec![],
            },
        )
        .await
        .unwrap();

    let environment = workspace
        .environment(&create_environment_output.id)
        .await
        .unwrap();

    let env_description = environment.describe(&ctx).await.unwrap();

    assert_eq!(env_description.name, new_environment_name);
    assert_eq!(env_description.variables.len(), 1);

    // TODO: check the database when it's implemented

    cleanup().await;
}
