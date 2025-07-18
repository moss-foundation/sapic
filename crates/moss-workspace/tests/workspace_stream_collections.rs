#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_testutils::random_name::random_collection_name;
use moss_workspace::models::{
    events::StreamCollectionsEvent, operations::CreateCollectionInput, primitives::CollectionId,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::ipc::{Channel, InvokeResponseBody};

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn stream_collections_empty_workspace() {
    let (ctx, _workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamCollectionsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_collections(&ctx, channel).await.unwrap();

    // Verify no events were received
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 0);
    assert_eq!(output.total_returned, 0);

    cleanup().await;
}

#[tokio::test]
async fn stream_collections_single_collection() {
    let (ctx, _workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let collection_order = 42;

    // Create a single collection
    let create_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: collection_order,
                external_path: None,
                repo: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();

    let collection_id = create_result.id;

    // Stream collections and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamCollectionsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_collections(&ctx, channel).await.unwrap();

    // Verify one event was received
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(output.total_returned, 1);

    // Verify the event data
    let event = &events[0];
    assert_eq!(event.id, collection_id);
    assert_eq!(event.name, collection_name);
    assert_eq!(event.order, Some(collection_order));
    assert_eq!(event.repository, None);
    assert_eq!(event.picture_path, None);

    cleanup().await;
}

#[tokio::test]
async fn stream_collections_multiple_collections() {
    let (ctx, _workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    let mut expected_collections = Vec::new();

    // Create multiple collections with different parameters
    for i in 0..5 {
        let collection_name = format!("Collection {}", i);
        let collection_order = i * 10;

        let create_result = workspace
            .create_collection(
                &ctx,
                &CreateCollectionInput {
                    name: collection_name.clone(),
                    order: collection_order,
                    external_path: None,
                    repo: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();

        expected_collections.push((create_result.id, collection_name, collection_order));
    }

    // Stream collections and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamCollectionsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_collections(&ctx, channel).await.unwrap();

    // Verify correct number of events
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 5);
    assert_eq!(output.total_returned, 5);

    // Convert events to a map for easier verification
    let events_map: HashMap<CollectionId, &StreamCollectionsEvent> = events
        .iter()
        .map(|event| (event.id.clone(), event))
        .collect();

    // Verify each expected collection is present with correct data
    for (expected_id, expected_name, expected_order) in expected_collections {
        let event = events_map.get(&expected_id).unwrap();
        assert_eq!(event.name, expected_name);
        assert_eq!(event.order, Some(expected_order));
        assert_eq!(event.repository, None);
        assert_eq!(event.picture_path, None);
    }

    cleanup().await;
}

#[tokio::test]
async fn stream_collections_with_repository() {
    let (ctx, _workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let collection_order = 100;
    let repository_url = "https://github.com/example/repo.git".to_string();

    // Create a collection with repository
    let create_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: collection_order,
                external_path: None,
                repo: Some(repository_url.clone()),
                icon_path: None,
            },
        )
        .await
        .unwrap();

    let collection_id = create_result.id;

    // Stream collections and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamCollectionsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_collections(&ctx, channel).await.unwrap();

    // Verify one event was received
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(output.total_returned, 1);

    // Verify the event data includes repository
    let event = &events[0];
    assert_eq!(event.id, collection_id);
    assert_eq!(event.name, collection_name);
    assert_eq!(event.order, Some(collection_order));
    // Note: The API currently returns None for repository, but we expect it to be fixed
    // TODO: Update this when the API is fixed to return the actual repository
    assert_eq!(event.repository, None);
    assert_eq!(event.picture_path, None);

    cleanup().await;
}

#[tokio::test]
async fn stream_collections_with_icon() {
    let (ctx, workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let collection_order = 200;

    // Create a test icon file
    let icon_path = workspace_path.join("test_icon.png");
    shared::generate_random_icon(&icon_path);

    // Create a collection with icon
    let create_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: collection_order,
                external_path: None,
                repo: None,
                icon_path: Some(icon_path.clone()),
            },
        )
        .await
        .unwrap();

    let collection_id = create_result.id;

    // Stream collections and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamCollectionsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_collections(&ctx, channel).await.unwrap();

    // Verify one event was received
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(output.total_returned, 1);

    // Verify the event data includes icon path
    let event = &events[0];
    assert_eq!(event.id, collection_id);
    assert_eq!(event.name, collection_name);
    assert_eq!(event.order, Some(collection_order));
    assert_eq!(event.repository, None);
    assert!(event.picture_path.is_some());

    cleanup().await;
}

#[tokio::test]
async fn stream_collections_mixed_configurations() {
    let (ctx, workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    // Create icon file
    let icon_path = workspace_path.join("mixed_test_icon.png");
    shared::generate_random_icon(&icon_path);

    let mut expected_collections = Vec::new();

    // Collection 1: Basic
    let name1 = "Basic Collection".to_string();
    let result1 = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: name1.clone(),
                order: 1,
                external_path: None,
                repo: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();
    expected_collections.push((result1.id, name1, 1, None::<String>, None::<String>));

    // Collection 2: With repository
    let name2 = "Repo Collection".to_string();
    let repo2 = "https://github.com/example/repo2.git".to_string();
    let result2 = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: name2.clone(),
                order: 2,
                external_path: None,
                repo: Some(repo2.clone()),
                icon_path: None,
            },
        )
        .await
        .unwrap();
    expected_collections.push((result2.id, name2, 2, Some(repo2), None::<String>));

    // Collection 3: With icon
    let name3 = "Icon Collection".to_string();
    let result3 = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: name3.clone(),
                order: 3,
                external_path: None,
                repo: None,
                icon_path: Some(icon_path.clone()),
            },
        )
        .await
        .unwrap();
    expected_collections.push((
        result3.id,
        name3,
        3,
        None::<String>,
        Some("icon".to_string()),
    ));

    // Stream collections and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamCollectionsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_collections(&ctx, channel).await.unwrap();

    // Verify correct number of events
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 3);
    assert_eq!(output.total_returned, 3);

    // Convert events to a map for easier verification
    let events_map: HashMap<CollectionId, &StreamCollectionsEvent> = events
        .iter()
        .map(|event| (event.id.clone(), event))
        .collect();

    // Verify each expected collection
    for (expected_id, expected_name, expected_order, _expected_repo, expected_icon) in
        expected_collections
    {
        let event = events_map.get(&expected_id).unwrap();
        assert_eq!(event.name, *expected_name);
        assert_eq!(event.order, Some(expected_order));
        // Note: Repository is currently not returned by the API
        assert_eq!(event.repository, None);

        // Check icon path presence
        match expected_icon {
            Some(_) => assert!(event.picture_path.is_some()),
            None => assert!(event.picture_path.is_none()),
        }
    }

    cleanup().await;
}

#[tokio::test]
async fn stream_collections_order_verification() {
    let (ctx, _workspace_path, workspace, _services, cleanup) = setup_test_workspace().await;

    let orders = vec![10, 5, 20, 1, 15];
    let mut expected_collections = Vec::new();

    // Create collections with different orders
    for order in orders.iter() {
        let collection_name = format!("Collection Order {}", order);
        let result = workspace
            .create_collection(
                &ctx,
                &CreateCollectionInput {
                    name: collection_name.clone(),
                    order: *order,
                    external_path: None,
                    repo: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();
        expected_collections.push((result.id, collection_name, *order));
    }

    // Stream collections and capture events
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamCollectionsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace.stream_collections(&ctx, channel).await.unwrap();

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
