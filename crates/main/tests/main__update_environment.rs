use crate::shared::set_up_test_main_window;
use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use moss_environment::models::types::{AddVariableParams, UpdateVariableParams, VariableOptions};
use moss_testutils::random_name::random_environment_name;
use sapic_base::{
    environment::types::primitives::{EnvironmentId, VariableId},
    project::types::primitives::ProjectId,
};
use sapic_ipc::contracts::main::{
    environment::{
        CreateEnvironmentInput, DescribeEnvironmentInput, DescribeEnvironmentOutput,
        UpdateEnvironmentInput, UpdateEnvironmentParams,
    },
    project::{CreateProjectInput, CreateProjectParams},
};
use serde_json::Value as JsonValue;

#[cfg(feature = "integration-tests")]
mod shared;

#[tokio::test]
async fn update_environment_workspace_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let create_input = CreateEnvironmentInput {
        project_id: None,
        name: random_environment_name(),
        order: 0,
        color: None,
        variables: vec![],
    };

    let env_id = main_window
        .create_environment(&ctx, create_input.clone())
        .await
        .unwrap()
        .id;

    // Update metadata
    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: None,
            id: env_id.clone(),
            name: Some("New Name".to_string()),
            order: None,
            color: Some(ChangeString::Update("#ffffff".to_string())),
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        },
    };

    main_window
        .update_environment(&ctx, input.clone())
        .await
        .unwrap();

    let desc = main_window
        .describe_environment(
            &ctx,
            &DescribeEnvironmentInput {
                project_id: None,
                environment_id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    assert_eq!(
        desc,
        DescribeEnvironmentOutput {
            name: "New Name".to_string(),
            color: Some("#ffffff".to_string()),
            variables: vec![],
        }
    );

    // Add variable
    let var_params = AddVariableParams {
        name: "Variable".to_string(),
        global_value: JsonValue::String("Global Value".to_string()),
        local_value: JsonValue::String("Local Value".to_string()),
        order: 0,
        desc: Some("Description".to_string()),
        options: VariableOptions { disabled: false },
    };
    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: None,
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![var_params.clone()],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        },
    };

    main_window
        .update_environment(&ctx, input.clone())
        .await
        .unwrap();

    let desc = main_window
        .describe_environment(
            &ctx,
            &DescribeEnvironmentInput {
                project_id: None,
                environment_id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    let var_desc = &desc.variables[0];

    assert_eq!(var_desc.name, var_params.name);
    assert_eq!(var_desc.global_value, Some(var_params.global_value));
    assert_eq!(var_desc.local_value, Some(var_params.local_value));
    assert_eq!(var_desc.desc, var_params.desc);
    assert_eq!(var_desc.disabled, var_params.options.disabled);

    // Update Variable
    let var_id = &var_desc.id;

    let update_variable_params = UpdateVariableParams {
        id: var_id.clone(),
        name: Some("Updated Variable".to_string()),
        global_value: Some(ChangeJsonValue::Update(JsonValue::String(
            "New Global Value".to_string(),
        ))),
        local_value: Some(ChangeJsonValue::Remove),
        order: None,
        desc: Some(ChangeString::Update("Updated Description".to_string())),
        options: Some(VariableOptions { disabled: true }),
    };

    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: None,
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![update_variable_params.clone()],
            vars_to_delete: vec![],
        },
    };

    main_window
        .update_environment(&ctx, input.clone())
        .await
        .unwrap();

    let desc = main_window
        .describe_environment(
            &ctx,
            &DescribeEnvironmentInput {
                project_id: None,
                environment_id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    let var_desc = &desc.variables[0];
    assert_eq!(var_desc.name, "Updated Variable".to_string());
    assert_eq!(
        var_desc.global_value,
        Some(JsonValue::String("New Global Value".to_string()))
    );
    assert_eq!(var_desc.local_value, None);
    assert_eq!(var_desc.desc, Some("Updated Description".to_string()));
    assert_eq!(var_desc.disabled, true);

    // Delete Variable
    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: None,
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![var_id.clone()],
        },
    };

    main_window
        .update_environment(&ctx, input.clone())
        .await
        .unwrap();

    let desc = main_window
        .describe_environment(
            &ctx,
            &DescribeEnvironmentInput {
                project_id: None,
                environment_id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    assert!(desc.variables.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn update_environment_workspace_nonexistent() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: None,
            id: EnvironmentId::new(),
            name: Some("New Name".to_string()),
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        },
    };

    let result = main_window.update_environment(&ctx, input.clone()).await;

    assert!(result.is_err());

    cleanup().await;
}

// Updating a non-existent variable will be an error
// While removing a non-existent one is a harmless no-op
#[tokio::test]
async fn update_environment_workspace_nonexistent_variables() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let create_input = CreateEnvironmentInput {
        project_id: None,
        name: random_environment_name(),
        order: 0,
        color: None,
        variables: vec![],
    };

    let env_id = main_window
        .create_environment(&ctx, create_input.clone())
        .await
        .unwrap()
        .id;

    // Update nonexistent variable
    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: None,
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![UpdateVariableParams {
                id: VariableId::new(),
                name: Some("New Name".to_string()),
                global_value: None,
                local_value: None,
                order: None,
                desc: None,
                options: None,
            }],
            vars_to_delete: vec![],
        },
    };

    let result = main_window.update_environment(&ctx, input.clone()).await;

    assert!(result.is_err());

    // Delete nonexistent variable

    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: None,
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![VariableId::new()],
        },
    };

    let result = main_window.update_environment(&ctx, input.clone()).await;
    assert!(result.is_ok());

    cleanup().await;
}

#[tokio::test]
async fn update_environment_project_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "New Project".to_string(),
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

    let create_input = CreateEnvironmentInput {
        project_id: Some(project_id.clone()),
        name: random_environment_name(),
        order: 0,
        color: None,
        variables: vec![],
    };

    let env_id = main_window
        .create_environment(&ctx, create_input.clone())
        .await
        .unwrap()
        .id;

    // Update metadata
    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: Some(project_id.clone()),
            id: env_id.clone(),
            name: Some("New Name".to_string()),
            order: None,
            color: Some(ChangeString::Update("#ffffff".to_string())),
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        },
    };

    main_window
        .update_environment(&ctx, input.clone())
        .await
        .unwrap();

    let desc = main_window
        .describe_environment(
            &ctx,
            &DescribeEnvironmentInput {
                project_id: Some(project_id.clone()),
                environment_id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    assert_eq!(
        desc,
        DescribeEnvironmentOutput {
            name: "New Name".to_string(),
            color: Some("#ffffff".to_string()),
            variables: vec![],
        }
    );

    // Add variable
    let var_params = AddVariableParams {
        name: "Variable".to_string(),
        global_value: JsonValue::String("Global Value".to_string()),
        local_value: JsonValue::String("Local Value".to_string()),
        order: 0,
        desc: Some("Description".to_string()),
        options: VariableOptions { disabled: false },
    };
    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: Some(project_id.clone()),
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![var_params.clone()],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        },
    };

    main_window
        .update_environment(&ctx, input.clone())
        .await
        .unwrap();

    let desc = main_window
        .describe_environment(
            &ctx,
            &DescribeEnvironmentInput {
                project_id: Some(project_id.clone()),
                environment_id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    let var_desc = &desc.variables[0];

    assert_eq!(var_desc.name, var_params.name);
    assert_eq!(var_desc.global_value, Some(var_params.global_value));
    assert_eq!(var_desc.local_value, Some(var_params.local_value));
    assert_eq!(var_desc.desc, var_params.desc);
    assert_eq!(var_desc.disabled, var_params.options.disabled);

    // Update Variable
    let var_id = &var_desc.id;

    let update_variable_params = UpdateVariableParams {
        id: var_id.clone(),
        name: Some("Updated Variable".to_string()),
        global_value: Some(ChangeJsonValue::Update(JsonValue::String(
            "New Global Value".to_string(),
        ))),
        local_value: Some(ChangeJsonValue::Remove),
        order: None,
        desc: Some(ChangeString::Update("Updated Description".to_string())),
        options: Some(VariableOptions { disabled: true }),
    };

    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: Some(project_id.clone()),
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![update_variable_params.clone()],
            vars_to_delete: vec![],
        },
    };

    main_window
        .update_environment(&ctx, input.clone())
        .await
        .unwrap();

    let desc = main_window
        .describe_environment(
            &ctx,
            &DescribeEnvironmentInput {
                project_id: Some(project_id.clone()),
                environment_id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    let var_desc = &desc.variables[0];
    assert_eq!(var_desc.name, "Updated Variable".to_string());
    assert_eq!(
        var_desc.global_value,
        Some(JsonValue::String("New Global Value".to_string()))
    );
    assert_eq!(var_desc.local_value, None);
    assert_eq!(var_desc.desc, Some("Updated Description".to_string()));
    assert_eq!(var_desc.disabled, true);

    // Delete Variable
    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: Some(project_id.clone()),
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![var_id.clone()],
        },
    };

    main_window
        .update_environment(&ctx, input.clone())
        .await
        .unwrap();

    let desc = main_window
        .describe_environment(
            &ctx,
            &DescribeEnvironmentInput {
                project_id: Some(project_id.clone()),
                environment_id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    assert!(desc.variables.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn update_environment_project_nonexistent_project() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: Some(ProjectId::new()),
            id: EnvironmentId::new(),
            name: Some("New Name".to_string()),
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        },
    };

    let result = main_window.update_environment(&ctx, input.clone()).await;

    assert!(result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn update_environment_project_nonexistent_environment() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "New Project".to_string(),
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

    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: Some(project_id.clone()),
            id: EnvironmentId::new(),
            name: Some("New Name".to_string()),
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![],
        },
    };

    let result = main_window.update_environment(&ctx, input.clone()).await;

    assert!(result.is_err());

    cleanup().await;
}

// Updating a non-existent variable will be an error
// While removing a non-existent one is a harmless no-op
#[tokio::test]
async fn update_environment_project_nonexistent_variables() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "New Project".to_string(),
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

    let create_input = CreateEnvironmentInput {
        project_id: Some(project_id.clone()),
        name: random_environment_name(),
        order: 0,
        color: None,
        variables: vec![],
    };

    let env_id = main_window
        .create_environment(&ctx, create_input.clone())
        .await
        .unwrap()
        .id;

    // Update nonexistent variable
    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: Some(project_id.clone()),
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![UpdateVariableParams {
                id: VariableId::new(),
                name: Some("New Name".to_string()),
                global_value: None,
                local_value: None,
                order: None,
                desc: None,
                options: None,
            }],
            vars_to_delete: vec![],
        },
    };

    let result = main_window.update_environment(&ctx, input.clone()).await;
    assert!(result.is_err());

    // Delete nonexistent variable
    let input = UpdateEnvironmentInput {
        inner: UpdateEnvironmentParams {
            project_id: Some(project_id.clone()),
            id: env_id.clone(),
            name: None,
            order: None,
            color: None,
            expanded: None,
            vars_to_add: vec![],
            vars_to_update: vec![],
            vars_to_delete: vec![VariableId::new()],
        },
    };

    let result = main_window.update_environment(&ctx, input.clone()).await;
    assert!(result.is_ok());

    cleanup().await;
}
