#![cfg(feature = "integration-tests")]

use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_environment::models::primitives::EnvironmentId;
use moss_testutils::random_name::{random_environment_name, random_project_name};
use moss_workspace::{
    Workspace,
    models::{
        events::StreamEnvironmentsEvent,
        operations::{ActivateEnvironmentInput, CreateEnvironmentInput, CreateProjectInput},
        types::CreateProjectParams,
    },
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::ipc::{Channel, InvokeResponseBody};

use crate::shared::setup_test_workspace;

pub mod shared;

async fn test_stream_environments<R: AppRuntime>(
    ctx: &R::AsyncContext,
    app_delegate: AppDelegate<R>,
    workspace: &Workspace<R>,
) -> HashMap<EnvironmentId, StreamEnvironmentsEvent> {
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamEnvironmentsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let _ = workspace
        .stream_environments(ctx, app_delegate, channel.clone())
        .await;
    received_events
        .lock()
        .unwrap()
        .iter()
        .map(|event| (event.id.clone(), event.clone()))
        .collect()
}

#[tokio::test]
async fn activate_environment_global() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name,
                project_id: None,
                order: 42,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let id = create_environment_output.id;

    let events_map = test_stream_environments(&ctx, app_delegate.clone(), &workspace).await;

    // Newly created environments are not automatically activated
    assert!(!events_map.get(&id).unwrap().is_active);
    workspace
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                environment_id: id.clone(),
            },
        )
        .await
        .unwrap();

    let events_map = test_stream_environments(&ctx, app_delegate.clone(), &workspace).await;

    assert!(events_map.get(&id).unwrap().is_active);

    cleanup().await;
}

#[tokio::test]
async fn activate_environment_collection() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_project_name();
    let collection_id = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: collection_name,
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

    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name,
                project_id: Some(collection_id.clone()),
                order: 42,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let id = create_environment_output.id;

    let events_map = test_stream_environments(&ctx, app_delegate.clone(), &workspace).await;

    // Newly created environments are not automatically activated
    assert!(!events_map.get(&id).unwrap().is_active);

    workspace
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                environment_id: id.clone(),
            },
        )
        .await
        .unwrap();

    let events_map = test_stream_environments(&ctx, app_delegate.clone(), &workspace).await;

    assert!(events_map.get(&id).unwrap().is_active);

    cleanup().await;
}

#[tokio::test]
async fn activate_environment_currently_active() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let environment_name = random_environment_name();
    let create_environment_output = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                name: environment_name,
                project_id: None,
                order: 0,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let id = create_environment_output.id;

    workspace
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                environment_id: id.clone(),
            },
        )
        .await
        .unwrap();

    // Try activating a currently active environment
    workspace
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                environment_id: id.clone(),
            },
        )
        .await
        .unwrap();

    let events_map = test_stream_environments(&ctx, app_delegate.clone(), &workspace).await;

    assert!(events_map.get(&id).unwrap().is_active);

    drop(workspace);
    cleanup().await;
}

// Activating environments for any group (including global) should not affect other groups
#[tokio::test]
async fn activate_environment_groups_isolation() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;
    let collection_name = random_project_name();
    let collection_id = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: collection_name,
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

    let collection_env_name = random_environment_name();
    let collection_env_id = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                project_id: Some(collection_id.clone()),
                name: collection_env_name,
                order: 0,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap()
        .id;

    let global_env_name = random_environment_name();
    let global_env_id = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                project_id: None,
                name: global_env_name,
                order: 0,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap()
        .id;

    workspace
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                environment_id: global_env_id.clone(),
            },
        )
        .await
        .unwrap();

    let events_map = test_stream_environments(&ctx, app_delegate.clone(), &workspace).await;

    assert!(events_map.get(&global_env_id).unwrap().is_active);
    assert!(!events_map.get(&collection_env_id).unwrap().is_active);
    cleanup().await;
}

#[tokio::test]
async fn activate_environment_nonexistent() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;
    let result = workspace
        .activate_environment(
            &ctx,
            ActivateEnvironmentInput {
                environment_id: EnvironmentId::new(),
            },
        )
        .await;

    assert!(result.is_err());
    cleanup().await;
}
