#![cfg(feature = "integration-tests")]

use moss_environment::models::types::{AddVariableParams, VariableOptions};
use moss_testutils::random_name::{random_environment_name, random_project_name};
use moss_workspace::models::{
    events::StreamEnvironmentsEvent,
    operations::{CreateEnvironmentInput, CreateProjectInput},
    types::CreateProjectParams,
};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::ipc::{Channel, InvokeResponseBody};

use crate::shared::setup_test_workspace;

pub mod shared;
#[tokio::test]
async fn stream_environments_no_custom_environment() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

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

    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();

    // There should be one predefined globals environment
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(output.total_returned, 1);
    // Since no collection environment is created, there is no environment group
    assert!(output.groups.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn stream_environments_only_workspace_environments() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let mut expected_environments = Vec::new();

    // Create multiple workspace environments

    for i in 0..5 {
        let environment_name = format!("Environment {}", i);
        let environment_order = i * 10;

        let create_result = workspace
            .create_environment(
                &ctx,
                app_delegate.clone(),
                CreateEnvironmentInput {
                    project_id: None,
                    name: environment_name.clone(),
                    order: environment_order,
                    color: None,
                    variables: vec![],
                },
            )
            .await
            .unwrap();

        expected_environments.push((create_result.id, environment_name, environment_order));
    }

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

    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();

    // 1 predefined + 5 workspace environment
    let events = received_events.lock().unwrap();

    assert_eq!(events.len(), 6);
    assert_eq!(output.total_returned, 6);
    assert!(output.groups.is_empty());

    let events_map: HashMap<_, _> = events
        .iter()
        .map(|event| (event.id.clone(), event.clone()))
        .collect();

    for (expected_id, expected_name, expected_order) in expected_environments {
        let event = events_map.get(&expected_id).unwrap();
        assert_eq!(event.name, expected_name);
        assert_eq!(event.order, Some(expected_order));
        assert_eq!(event.project_id, None);
        assert_eq!(event.total_variables, 0);
    }

    cleanup().await;
}

#[tokio::test]
async fn stream_environments_only_collection_environments() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let mut expected_environments = Vec::new();

    // Create multiple collections, each with an environment

    for i in 0..5 {
        let collection_name = format!("Collection {}", i);
        let collection_id = workspace
            .create_project(
                &ctx,
                &app_delegate,
                &CreateProjectInput {
                    inner: CreateProjectParams {
                        name: collection_name,
                        order: i,
                        external_path: None,
                        git_params: None,
                        icon_path: None,
                    },
                },
            )
            .await
            .unwrap()
            .id;

        let environment_name = format!("Environment {}", i);
        let environment_order = i * 10;
        let create_result = workspace
            .create_environment(
                &ctx,
                app_delegate.clone(),
                CreateEnvironmentInput {
                    project_id: Some(collection_id.clone()),
                    name: environment_name.clone(),
                    order: environment_order,
                    color: None,
                    variables: vec![],
                },
            )
            .await
            .unwrap();

        expected_environments.push((
            create_result.id,
            environment_name,
            environment_order,
            collection_id,
        ))
    }

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

    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();

    // 1 predefined + 5 collection environment
    let events = received_events.lock().unwrap();

    assert_eq!(events.len(), 6);
    assert_eq!(output.total_returned, 6);
    // One EnvironmentGroup for each collection
    assert_eq!(output.groups.len(), 5);

    let events_map: HashMap<_, _> = events
        .iter()
        .map(|event| (event.id.clone(), event.clone()))
        .collect();

    let groups_map: HashMap<_, _> = output
        .groups
        .iter()
        .map(|group| (group.project_id.clone(), group))
        .collect();

    for (expected_id, expected_name, expected_order, expected_group) in expected_environments {
        let event = events_map.get(&expected_id).unwrap();
        assert_eq!(event.name, expected_name);
        assert_eq!(event.order, Some(expected_order));
        assert_eq!(event.project_id, Some(expected_group.clone()));
        assert_eq!(event.total_variables, 0);

        let group = groups_map.get(&expected_group.inner()).unwrap();

        // Newly created EnvironmentGroups should be expanded by default
        assert!(group.expanded)
    }

    cleanup().await;
}

#[tokio::test]
async fn stream_environments_both_workspace_and_collection_environments() {
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
    let collection_env_result = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                project_id: Some(collection_id.clone()),
                name: collection_env_name.clone(),
                order: 0,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

    let workspace_env_name = random_environment_name();
    let workspace_env_result = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                project_id: None,
                name: workspace_env_name.clone(),
                order: 0,
                color: None,
                variables: vec![],
            },
        )
        .await
        .unwrap();

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

    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();

    // 1 predefined + 1 workspace + 1 collection environment
    let events = received_events.lock().unwrap();

    assert_eq!(events.len(), 3);
    assert_eq!(output.total_returned, 3);
    // Only one group for the collection
    assert_eq!(output.groups.len(), 1);

    assert_eq!(output.groups[0].project_id, collection_id.clone().inner());
    assert_eq!(output.groups[0].expanded, true);

    let events_map: HashMap<_, _> = events
        .iter()
        .map(|event| (event.id.clone(), event.clone()))
        .collect();

    let collection_env_event = events_map.get(&collection_env_result.id).unwrap();
    assert_eq!(collection_env_event.name, collection_env_name);
    assert_eq!(collection_env_event.order, Some(0));
    assert_eq!(collection_env_event.project_id, Some(collection_id.clone()));
    assert_eq!(collection_env_event.total_variables, 0);

    let workspace_env_event = events_map.get(&workspace_env_result.id).unwrap();
    assert_eq!(workspace_env_event.name, workspace_env_name);
    assert_eq!(workspace_env_event.order, Some(0));
    assert_eq!(workspace_env_event.project_id, None);
    assert_eq!(workspace_env_event.total_variables, 0);

    cleanup().await;
}

#[tokio::test]
async fn stream_environments_with_variables() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let variables = (0..5)
        .map(|i| AddVariableParams {
            name: format!("variable{}", i),
            global_value: JsonValue::Number(i.clone().into()),
            local_value: JsonValue::Number(i.clone().into()),
            order: i,
            desc: None,
            options: VariableOptions { disabled: false },
        })
        .collect::<Vec<_>>();

    let environment_name = random_environment_name();
    let environment_result = workspace
        .create_environment(
            &ctx,
            app_delegate.clone(),
            CreateEnvironmentInput {
                project_id: None,
                name: environment_name.clone(),
                order: 0,
                color: None,
                variables,
            },
        )
        .await
        .unwrap();

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

    let output = workspace
        .stream_environments(&ctx, app_delegate.clone(), channel)
        .await
        .unwrap();

    // 1 predefined + 1 workspace environment
    let events = received_events.lock().unwrap();

    assert_eq!(events.len(), 2);
    assert_eq!(output.total_returned, 2);
    assert!(output.groups.is_empty());

    let events_map: HashMap<_, _> = events
        .iter()
        .map(|event| (event.id.clone(), event.clone()))
        .collect();
    let event = events_map.get(&environment_result.id).unwrap();
    assert_eq!(event.name, environment_name);
    assert_eq!(event.order, Some(0));
    assert_eq!(event.project_id, None);
    assert_eq!(event.total_variables, 5);

    cleanup().await;
}
