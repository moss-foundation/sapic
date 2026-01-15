use crate::shared::{set_up_test_main_window, test_stream_environments};
use moss_testutils::random_name::random_environment_name;
use sapic_base::{
    environment::types::primitives::EnvironmentId, project::types::primitives::ProjectId,
};
use sapic_ipc::contracts::main::{
    environment::{CreateEnvironmentInput, DeleteEnvironmentInput},
    project::{CreateProjectInput, CreateProjectParams},
};

#[cfg(feature = "integration-tests")]
mod shared;

#[tokio::test]
async fn delete_environment_workspace_success() {
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

    main_window
        .delete_environment(
            &ctx,
            DeleteEnvironmentInput {
                project_id: None,
                id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    // Check that no environment exists
    let environments = test_stream_environments(&ctx, &main_window, None).await;

    assert_eq!(environments.len(), 0);

    cleanup().await
}

// This should be handled gracefully
#[tokio::test]
async fn delete_environment_workspace_nonexistent() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let result = main_window
        .delete_environment(
            &ctx,
            DeleteEnvironmentInput {
                project_id: None,
                id: EnvironmentId::new(),
            },
        )
        .await;

    assert!(result.is_ok());

    cleanup().await
}

#[tokio::test]
async fn delete_environment_project_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "Project".to_string(),
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

    main_window
        .delete_environment(
            &ctx,
            DeleteEnvironmentInput {
                project_id: Some(project_id.clone()),
                id: env_id.clone(),
            },
        )
        .await
        .unwrap();

    // Check that no environment exists
    let environments = test_stream_environments(&ctx, &main_window, Some(project_id.clone())).await;

    assert_eq!(environments.len(), 0);

    cleanup().await
}

// Trying to delete environment from a nonexistent project is an unlikely error
#[tokio::test]
async fn delete_environment_project_nonexistent_project() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let result = main_window
        .delete_environment(
            &ctx,
            DeleteEnvironmentInput {
                project_id: Some(ProjectId::new()),
                id: EnvironmentId::new(),
            },
        )
        .await;

    assert!(result.is_err());
    cleanup().await
}

// Deleting a nonexistent environment from a project should be handled gracefully
#[tokio::test]
async fn delete_environment_project_nonexistent_environment() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "Project".to_string(),
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

    let result = main_window
        .delete_environment(
            &ctx,
            DeleteEnvironmentInput {
                project_id: Some(project_id.clone()),
                id: EnvironmentId::new(),
            },
        )
        .await;

    assert!(result.is_ok());
    cleanup().await
}
