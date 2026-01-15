use crate::shared::{set_up_test_main_window, test_stream_environments};
use moss_environment::models::types::{AddVariableParams, VariableOptions};
use moss_testutils::random_name::random_environment_name;
use sapic_base::project::types::primitives::ProjectId;
use sapic_ipc::contracts::main::{
    environment::{
        CreateEnvironmentInput, CreateEnvironmentOutput, DescribeEnvironmentInput,
        StreamEnvironmentsEvent,
    },
    project::{CreateProjectInput, CreateProjectParams},
};
use serde_json::Value as JsonValue;

#[cfg(feature = "integration-tests")]
mod shared;

#[tokio::test]
async fn create_environment_workspace_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let var_params = AddVariableParams {
        name: "Variable".to_string(),
        global_value: JsonValue::String("Global Value".to_string()),
        local_value: JsonValue::String("Local Value".to_string()),
        order: 0,
        desc: Some("Description".to_string()),
        options: VariableOptions { disabled: false },
    };
    let create_input = CreateEnvironmentInput {
        project_id: None,
        name: random_environment_name(),
        order: 0,
        color: None,
        variables: vec![var_params.clone()],
    };

    let env_id = main_window
        .create_environment(&ctx, create_input.clone())
        .await
        .unwrap()
        .id;

    // Check the environment is returned in stream
    let environments = test_stream_environments(&ctx, &main_window, None).await;

    assert_eq!(environments.len(), 1);
    assert_eq!(
        environments.get(&env_id).unwrap(),
        &StreamEnvironmentsEvent {
            id: env_id.clone(),
            project_id: None,
            is_active: false,
            name: create_input.name.clone(),
            color: None,
            order: None,
            total_variables: 1,
        }
    );

    // Check the variable is correctly created
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

    assert_eq!(desc.variables.len(), 1);
    assert_eq!(desc.variables[0].name, var_params.name);
    assert_eq!(
        desc.variables[0].global_value,
        Some(var_params.global_value)
    );
    assert_eq!(desc.variables[0].local_value, Some(var_params.local_value));
    assert_eq!(desc.variables[0].desc, var_params.desc);
    assert_eq!(desc.variables[0].disabled, var_params.options.disabled);

    cleanup().await;
}

#[tokio::test]
async fn create_environment_project_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "Test Project".to_string(),
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

    let var_params = AddVariableParams {
        name: "Variable".to_string(),
        global_value: JsonValue::String("Global Value".to_string()),
        local_value: JsonValue::String("Local Value".to_string()),
        order: 0,
        desc: Some("Description".to_string()),
        options: VariableOptions { disabled: false },
    };
    let create_input = CreateEnvironmentInput {
        project_id: Some(project_id.clone()),
        name: random_environment_name(),
        order: 0,
        color: None,
        variables: vec![var_params.clone()],
    };

    let env_id = main_window
        .create_environment(&ctx, create_input.clone())
        .await
        .unwrap()
        .id;

    // Check the environment is returned in stream
    let environments = test_stream_environments(&ctx, &main_window, Some(project_id.clone())).await;

    assert_eq!(environments.len(), 1);
    assert_eq!(
        environments.get(&env_id).unwrap(),
        &StreamEnvironmentsEvent {
            id: env_id.clone(),
            project_id: Some(project_id.clone()),
            is_active: false,
            name: create_input.name.clone(),
            color: None,
            order: None,
            total_variables: 1,
        }
    );

    // Check the variable is correctly created
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

    assert_eq!(desc.variables.len(), 1);
    assert_eq!(desc.variables[0].name, var_params.name);
    assert_eq!(
        desc.variables[0].global_value,
        Some(var_params.global_value)
    );
    assert_eq!(desc.variables[0].local_value, Some(var_params.local_value));
    assert_eq!(desc.variables[0].desc, var_params.desc);
    assert_eq!(desc.variables[0].disabled, var_params.options.disabled);

    cleanup().await;
}

#[tokio::test]
async fn create_environment_project_nonexistent_project() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let create_input = CreateEnvironmentInput {
        project_id: Some(ProjectId::new()),
        name: random_environment_name(),
        order: 0,
        color: None,
        variables: vec![],
    };

    let result = main_window
        .create_environment(&ctx, create_input.clone())
        .await;

    assert!(result.is_err());

    cleanup().await;
}
