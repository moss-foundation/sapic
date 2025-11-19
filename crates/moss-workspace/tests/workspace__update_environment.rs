#![cfg(feature = "integration-tests")]
pub mod shared;

use crate::shared::setup_test_workspace;
use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use moss_environment::{
    AnyEnvironment,
    models::{
        primitives::VariableId,
        types::{AddVariableParams, UpdateVariableParams, VariableOptions},
    },
};
use moss_storage2::{Storage, models::primitives::StorageScope};
use moss_testutils::random_name::random_environment_name;
use moss_workspace::{
    models::{
        operations::{CreateEnvironmentInput, UpdateEnvironmentInput},
        types::UpdateEnvironmentParams,
    },
    storage::key_environment_order,
};
use serde_json::Value as JsonValue;
// TODO: Test updating collection_id once it's implemented
// TODO: Update test once we switch variable store to new database

#[tokio::test]
async fn update_environment_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let old_environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: old_environment_name.clone(),
                project_id: None,
                order: 0,
                color: Some("#ffffff".to_string()),
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let new_environment_name = random_environment_name();

    let _ = workspace
        .update_environment(
            &ctx,
            UpdateEnvironmentInput {
                inner: UpdateEnvironmentParams {
                    id: create_environment_output.id.clone(),
                    name: Some(new_environment_name.clone()),
                    order: Some(42),
                    color: Some(ChangeString::Update("#000000".to_string())),
                    expanded: Some(false),
                    vars_to_add: vec![AddVariableParams {
                        name: "1".to_string(),
                        global_value: JsonValue::String("variable 1".to_string()),
                        local_value: JsonValue::String("variable 1".to_string()),
                        order: 1,
                        desc: Some("First variable".to_string()),
                        options: VariableOptions { disabled: true },
                    }],
                    vars_to_update: vec![],
                    vars_to_delete: vec![],
                },
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

    let id = create_environment_output.id.clone();
    // Check db is updated
    let storage = <dyn Storage>::global(&app_delegate);
    // Check environment cache is updated with new order and expanded
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

    let color = env_description.color;
    assert_eq!(color, Some("#000000".to_string()));

    cleanup().await;
}

#[tokio::test]
async fn update_environment_add_variables() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;
    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: None,
                order: 0,
                color: Some("#ffffff".to_string()),
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let id = create_environment_output.id.clone();

    let var1 = AddVariableParams {
        name: "1".to_string(),
        global_value: JsonValue::String("variable 1".to_string()),
        local_value: JsonValue::String("variable 1".to_string()),
        order: 1,
        desc: Some("First variable".to_string()),
        options: VariableOptions { disabled: true },
    };

    let var2 = AddVariableParams {
        name: "2".to_string(),
        global_value: JsonValue::String("${var1}".to_string()),
        local_value: JsonValue::String("${var1}".to_string()),
        order: 2,
        desc: Some("Second variable".to_string()),
        options: VariableOptions { disabled: false },
    };

    let _ = workspace
        .update_environment(
            &ctx,
            UpdateEnvironmentInput {
                inner: UpdateEnvironmentParams {
                    id: id.clone(),
                    name: None,
                    order: None,
                    color: None,
                    expanded: None,
                    vars_to_add: vec![var1.clone(), var2.clone()],
                    vars_to_delete: vec![],
                    vars_to_update: vec![],
                },
            },
        )
        .await
        .unwrap();

    let environment = workspace.environment(&id).await.unwrap();

    // Check that the variables are correctly added
    let env_description = environment.describe(&ctx).await.unwrap();

    let variables = env_description.variables;

    assert_eq!(variables.len(), 2);

    assert!(variables.clone().into_iter().any(|(_, var)| {
        var.name == var1.name
            && var.global_value == Some(var1.global_value.clone())
            && var.local_value == Some(var1.local_value.clone())
            && var.order == Some(var1.order)
            && var.desc == var1.desc
            && var.disabled == var1.options.disabled
    }));

    assert!(variables.clone().into_iter().any(|(_, var)| {
        var.name == var2.name
            && var.global_value == Some(var2.global_value.clone())
            && var.local_value == Some(var2.local_value.clone())
            && var.order == Some(var2.order.clone())
            && var.desc == var2.desc
            && var.disabled == var2.options.disabled
    }));

    cleanup().await;
}

#[tokio::test]
async fn update_environment_update_variables() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;
    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: None,
                order: 0,
                color: Some("#ffffff".to_string()),
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let environment_id = create_environment_output.id.clone();

    let old = AddVariableParams {
        name: "1".to_string(),
        global_value: JsonValue::String("variable 1".to_string()),
        local_value: JsonValue::String("variable 1".to_string()),
        order: 1,
        desc: Some("First variable".to_string()),
        options: VariableOptions { disabled: true },
    };

    // Add a variable to be updated
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
                    vars_to_add: vec![old],
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

    let new_name = "New Name".to_string();
    let new_global_value = JsonValue::String("New Global Value".to_string());
    let new_order = 42;
    let new_desc = "New description".to_string();

    let new = UpdateVariableParams {
        id: variable_id,
        name: Some(new_name.clone()),
        global_value: Some(ChangeJsonValue::Update(new_global_value.clone())),
        local_value: Some(ChangeJsonValue::Remove),
        order: Some(new_order),
        desc: Some(ChangeString::Update(new_desc.clone())),
        options: Some(VariableOptions { disabled: true }),
    };

    // Update an existing variable
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
                    vars_to_add: vec![],
                    vars_to_update: vec![new.clone()],
                    vars_to_delete: vec![],
                },
            },
        )
        .await
        .unwrap();

    // Check that the variable is correctly updated
    let env_description = workspace
        .environment(&environment_id)
        .await
        .unwrap()
        .describe(&ctx)
        .await
        .unwrap();

    let variables = env_description.variables;

    assert!(variables.into_iter().any(|(_, var)| {
        var.name == new_name
        && var.global_value == Some(new_global_value.clone())
        && var.local_value == None // We removed the local_value
        && var.order == Some(new_order.clone())
        && var.desc == Some(new_desc.clone())
        && var.disabled == true
    }));

    cleanup().await;
}

#[tokio::test]
async fn update_environment_update_variables_nonexistent() {
    // Trying to update a nonexistent variable should raise an error

    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;
    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: None,
                order: 0,
                color: Some("#ffffff".to_string()),
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let environment_id = create_environment_output.id.clone();

    let update_params = UpdateVariableParams {
        id: VariableId::new(),
        name: Some("New Name".to_string()),
        global_value: None,
        local_value: None,
        order: None,
        desc: None,
        options: None,
    };
    let result = workspace
        .update_environment(
            &ctx,
            UpdateEnvironmentInput {
                inner: UpdateEnvironmentParams {
                    id: environment_id.clone(),
                    name: None,
                    order: None,
                    color: None,
                    expanded: None,
                    vars_to_add: vec![],
                    vars_to_update: vec![update_params.clone()],
                    vars_to_delete: vec![],
                },
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn update_environment_delete_variables() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;
    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: None,
                order: 0,
                color: Some("#ffffff".to_string()),
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let environment_id = create_environment_output.id.clone();

    let var1 = AddVariableParams {
        name: "1".to_string(),
        global_value: JsonValue::String("variable 1".to_string()),
        local_value: JsonValue::String("variable 1".to_string()),
        order: 1,
        desc: Some("First variable".to_string()),
        options: VariableOptions { disabled: true },
    };

    // Add a variable to be deleted
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
                    vars_to_add: vec![var1.clone()],
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

    // Delete the variable
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
                    vars_to_add: vec![],
                    vars_to_update: vec![],
                    vars_to_delete: vec![variable_id],
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

    assert!(variables.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn update_environment_delete_variables_nonexistent() {
    // Delete a nonexistent variable should be a no-op

    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;
    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate,
            CreateEnvironmentInput {
                name: environment_name.clone(),
                project_id: None,
                order: 0,
                color: Some("#ffffff".to_string()),
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let environment_id = create_environment_output.id.clone();

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
                    vars_to_add: vec![],
                    vars_to_update: vec![],
                    vars_to_delete: vec![VariableId::new()],
                },
            },
        )
        .await
        .unwrap();

    cleanup().await;
}
