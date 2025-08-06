#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_bindingutils::primitives::ChangeString;
use moss_environment::{
    AnyEnvironment,
    models::types::{AddVariableParams, VariableOptions},
};
use moss_storage::storage::operations::GetItem;
use moss_testutils::random_name::random_environment_name;
use moss_workspace::{
    models::operations::{CreateEnvironmentInput, UpdateEnvironmentInput},
    storage::segments::SEGKEY_ENVIRONMENT,
};
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
                order: Some(42),
                color: Some(ChangeString::Update("#000000".to_string())),
                expanded: Some(false),
                vars_to_add: vec![AddVariableParams {
                    name: "TEST_VAR".to_string(),
                    global_value: JsonValue::String("test".to_string()),
                    local_value: JsonValue::String("test".to_string()),
                    order: 42,
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

    // Check environment cache is updated with new order and expanded value
    let item_store = workspace.db().item_store();

    let id = create_environment_output.id.clone();
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

    let stored_env_expanded: bool = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_ENVIRONMENT.join(id.as_str()).join("expanded"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(stored_env_expanded, false);

    // Check variables are updated
    let env_desc = workspace
        .environment(&create_environment_output.id)
        .await
        .unwrap()
        .describe(&ctx)
        .await
        .unwrap();
    let variables = env_desc.variables;

    assert_eq!(variables.len(), 1);

    // Check local_value and order are correctly restored from the database
    assert_eq!(
        variables[0].local_value,
        Some(JsonValue::String("test".to_string()))
    );
    assert_eq!(variables[0].order, 42);

    cleanup().await;
}
