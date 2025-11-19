#![cfg(feature = "integration-tests")]
pub mod shared;

use crate::shared::setup_test_workspace;
use moss_project::models::primitives::ProjectId;
use moss_testutils::random_name::random_project_name;
use moss_workspace::models::{
    events::StreamProjectsEvent, operations::CreateProjectInput, types::CreateProjectParams,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::ipc::{Channel, InvokeResponseBody};

#[tokio::test]
async fn stream_projects_empty_workspace() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamProjectsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_projects(&ctx, channel).await.unwrap();

    // Verify no events were received
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 0);
    assert_eq!(output.total_returned, 0);

    cleanup().await;
}

#[tokio::test]
async fn stream_projects_single_project() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = random_project_name();
    let project_order = 42;

    // Create a single collection
    let create_result = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: project_order,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    let project_id = create_result.id;

    // Stream projects and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamProjectsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_projects(&ctx, channel).await.unwrap();

    // Verify one event was received
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(output.total_returned, 1);

    // Verify the event data
    let event = &events[0];
    assert_eq!(event.id, project_id);
    assert_eq!(event.name, project_name);
    assert_eq!(event.order, Some(project_order));
    assert_eq!(event.icon_path, None);

    cleanup().await;
}

#[tokio::test]
async fn stream_projects_multiple_projects() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let mut expected_projects = Vec::new();

    // Create multiple projects with different parameters
    for i in 0..5 {
        let project_name = format!("Project {}", i);
        let project_order = i * 10;

        let create_result = workspace
            .create_project(
                &ctx,
                &app_delegate,
                &CreateProjectInput {
                    inner: CreateProjectParams {
                        name: project_name.clone(),
                        order: project_order,
                        external_path: None,
                        git_params: None,
                        icon_path: None,
                    },
                },
            )
            .await
            .unwrap();

        expected_projects.push((create_result.id, project_name, project_order));
    }

    // Stream projects and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamProjectsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_projects(&ctx, channel).await.unwrap();

    // Verify correct number of events
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 5);
    assert_eq!(output.total_returned, 5);

    // Convert events to a map for easier verification
    let events_map: HashMap<ProjectId, &StreamProjectsEvent> = events
        .iter()
        .map(|event| (event.id.clone(), event))
        .collect();

    // Verify each expected project is present with correct data
    for (expected_id, expected_name, expected_order) in expected_projects {
        let event = events_map.get(&expected_id).unwrap();
        assert_eq!(event.name, expected_name);
        assert_eq!(event.order, Some(expected_order));
        assert_eq!(event.icon_path, None);
    }

    cleanup().await;
}

#[tokio::test]
async fn stream_projects_with_icon() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = random_project_name();
    let project_order = 200;

    // Create a test icon file
    let icon_path = workspace.abs_path().join("test_icon.png");
    shared::generate_random_icon(&icon_path);

    // Create a collection with icon
    let create_result = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: project_order,
                    external_path: None,
                    git_params: None,
                    icon_path: Some(icon_path.clone()),
                },
            },
        )
        .await
        .unwrap();

    let project_id = create_result.id;

    // Stream projects and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamProjectsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_projects(&ctx, channel).await.unwrap();

    // Verify one event was received
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(output.total_returned, 1);

    // Verify the event data includes icon path
    let event = &events[0];
    assert_eq!(event.id, project_id);
    assert_eq!(event.name, project_name);
    assert_eq!(event.order, Some(project_order));
    assert!(event.icon_path.is_some());

    cleanup().await;
}

#[tokio::test]
async fn stream_projects_mixed_configurations() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    // Create icon file
    let icon_path = workspace.abs_path().join("mixed_test_icon.png");
    shared::generate_random_icon(&icon_path);

    let mut expected_projects = Vec::new();

    // Project 1: Basic
    let name1 = "Basic Project".to_string();
    let result1 = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: name1.clone(),
                    order: 1,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();
    expected_projects.push((result1.id, name1, 1, None::<String>));

    // Project 2: With icon
    let name2 = "Icon Project".to_string();
    let result2 = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: name2.clone(),
                    order: 2,
                    external_path: None,
                    git_params: None,
                    icon_path: Some(icon_path.clone()),
                },
            },
        )
        .await
        .unwrap();
    expected_projects.push((result2.id, name2, 2, Some("icon".to_string())));

    // Stream projects and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamProjectsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_projects(&ctx, channel).await.unwrap();

    // Verify correct number of events
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(output.total_returned, 2);

    // Convert events to a map for easier verification
    let events_map: HashMap<ProjectId, &StreamProjectsEvent> = events
        .iter()
        .map(|event| (event.id.clone(), event))
        .collect();

    // Verify each expected project
    for (expected_id, expected_name, expected_order, expected_icon) in expected_projects {
        let event = events_map.get(&expected_id).unwrap();
        assert_eq!(event.name, *expected_name);
        assert_eq!(event.order, Some(expected_order));

        // Check icon path presence
        match expected_icon {
            Some(_) => assert!(event.icon_path.is_some()),
            None => assert!(event.icon_path.is_none()),
        }
    }

    cleanup().await;
}

#[tokio::test]
async fn stream_projects_order_verification() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let orders = vec![10, 5, 20, 1, 15];
    let mut expected_projects = Vec::new();

    // Create projects with different orders
    for order in orders.iter() {
        let project_name = format!("Project Order {}", order);
        let result = workspace
            .create_project(
                &ctx,
                &app_delegate,
                &CreateProjectInput {
                    inner: CreateProjectParams {
                        name: project_name.clone(),
                        order: *order,
                        external_path: None,
                        git_params: None,
                        icon_path: None,
                    },
                },
            )
            .await
            .unwrap();
        expected_projects.push((result.id, project_name, *order));
    }

    // Stream projects and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamProjectsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_projects(&ctx, channel).await.unwrap();

    // Verify correct number of events
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 5);
    assert_eq!(output.total_returned, 5);

    // Verify all orders are present
    let received_orders: Vec<isize> = events.iter().map(|event| event.order.unwrap()).collect();

    for expected_order in &orders {
        assert!(received_orders.contains(expected_order));
    }

    cleanup().await;
}

// FIXME: figure out how to incorporate repo-operations into CI pipeline

// #[tokio::test]
// async fn stream_collections_with_repository() {
//     let (ctx, workspace, cleanup) = setup_test_workspace().await;
//
//     let project_name = random_project_name();
//     let project_order = 100;
//     let repository_url =
//         "https://github.com/brutusyhy/test-sapic-collection-private.git".to_string();
//
//     // Create a collection with repository
//     let create_result = workspace
//         .create_collection(
//             &ctx,
//             &CreateCollectionInput {
//                 name: project_name.clone(),
//                 order: project_order,
//                 external_path: None,
//                 repository: Some(repository_url.clone()),
//                 git_provider_type: Some(GitProviderType::GitHub),
//                 icon_path: None,
//             },
//         )
//         .await
//         .unwrap();
//
//     let project_id = create_result.id;
//
//     // Stream projects and capture events
//     let received_events = Arc::new(Mutex::new(Vec::new()));
//     let received_events_clone = received_events.clone();
//
//     let channel = Channel::new(move |body: InvokeResponseBody| {
//         if let InvokeResponseBody::Json(json_str) = body {
//             if let Ok(event) = serde_json::from_str::<StreamCollectionsEvent>(&json_str) {
//                 received_events_clone.lock().unwrap().push(event);
//             }
//         }
//         Ok(())
//     });
//
//     let output = workspace.stream_collections(&ctx, channel).await.unwrap();
//
//     // Verify one event was received
//     let events = received_events.lock().unwrap();
//     assert_eq!(events.len(), 1);
//     assert_eq!(output.total_returned, 1);
//
//     // Verify the event data includes repository
//     let event = &events[0];
//     assert_eq!(event.id, project_id);
//     assert_eq!(event.name, project_name);
//     assert_eq!(event.order, Some(project_order));
//     assert_eq!(
//         event.repository,
//         Some("https://github.com/brutusyhy/test-sapic-collection-private.git".to_string())
//     );
//     // Verify the API call succeeded
//     assert!(event.repository_info.is_some());
//
//     assert_eq!(event.icon_path, None);
//
//     cleanup().await;
// }
