use main::MainWindow;
use moss_applib::AppRuntime;
use moss_testutils::random_name::random_project_name;
use sapic_base::{
    environment::types::primitives::EnvironmentId, project::types::primitives::ProjectId,
};
use sapic_core::context::AnyAsyncContext;
use sapic_ipc::contracts::main::{
    environment::{
        ActivateEnvironmentInput, CreateEnvironmentInput, StreamEnvironmentsEvent,
        StreamProjectEnvironmentsInput,
    },
    project::{CreateProjectInput, CreateProjectParams},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::ipc::{Channel, InvokeResponseBody};

use crate::shared::{set_up_test_main_window, test_stream_environments};

#[cfg(feature = "integration-tests")]
mod shared;

#[tokio::test]
pub async fn activate_environment_workspace_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let create_input = CreateEnvironmentInput {
        project_id: None,
        name: "New Environment".to_string(),
        order: 0,
        color: None,
        variables: vec![],
    };

    let id = main_window
        .create_environment(&ctx, create_input.clone())
        .await
        .unwrap()
        .id;

    // By default, newly created environment is not activated.
    // The frontend will send a separate activate_environment operation after creation when necessary
    let environments = test_stream_environments(&ctx, &main_window, None).await;

    assert!(!environments.get(&id).unwrap().is_active);

    // Try activating the environment
    main_window
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                project_id: None,
                environment_id: id.clone(),
            },
        )
        .await
        .unwrap();

    let environments = test_stream_environments(&ctx, &main_window, None).await;

    assert!(environments.get(&id).unwrap().is_active);

    // Try activating it again, this should be a successful no-op
    main_window
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                project_id: None,
                environment_id: id.clone(),
            },
        )
        .await
        .unwrap();

    let environments = test_stream_environments(&ctx, &main_window, None).await;

    assert!(environments.get(&id).unwrap().is_active);

    cleanup().await;
}

#[tokio::test]
pub async fn activate_environment_workspace_nonexistent() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let result = main_window
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                project_id: None,
                environment_id: EnvironmentId::new(),
            },
        )
        .await;

    assert!(result.is_err());
    cleanup().await;
}

#[tokio::test]
pub async fn activate_environment_project_success() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_name = random_project_name();
    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.to_string(),
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
        name: "New Environment".to_string(),
        order: 0,
        color: None,
        variables: vec![],
    };

    let environment_id = main_window
        .create_environment(&ctx, create_input.clone())
        .await
        .unwrap()
        .id;

    // By default, newly created environment is not activated.
    // The frontend will send a separate activate_environment operation after creation when necessary
    let environments = test_stream_environments(&ctx, &main_window, Some(project_id.clone())).await;

    assert!(!environments.get(&environment_id).unwrap().is_active);

    // Try activating the environment
    main_window
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                project_id: Some(project_id.clone()),
                environment_id: environment_id.clone(),
            },
        )
        .await
        .unwrap();

    let environments = test_stream_environments(&ctx, &main_window, Some(project_id.clone())).await;

    assert!(environments.get(&environment_id).unwrap().is_active);

    // Try activating it again
    // This should be a successful no-op
    main_window
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                project_id: Some(project_id.clone()),
                environment_id: environment_id.clone(),
            },
        )
        .await
        .unwrap();

    let environments = test_stream_environments(&ctx, &main_window, Some(project_id.clone())).await;

    assert!(environments.get(&environment_id).unwrap().is_active);

    cleanup().await;
}

#[tokio::test]
pub async fn activate_environment_project_nonexistent_project() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;
    let result = main_window
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                project_id: Some(ProjectId::new()),
                environment_id: EnvironmentId::new(),
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}

#[tokio::test]
pub async fn activate_environment_project_nonexistent_environment() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_name = random_project_name();
    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.to_string(),
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
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                project_id: Some(project_id.clone()),
                environment_id: EnvironmentId::new(),
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}
