#![cfg(feature = "integration-tests")]
pub mod shared;

use futures;
use moss_app_delegate::AppDelegate;
use moss_applib::{context::AsyncContext, mock::MockAppRuntime};
use moss_collection::{
    dirs,
    models::{events::StreamEntriesEvent, operations::StreamEntriesInput, primitives::EntryKind},
};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::ipc::{Channel as TauriChannel, InvokeResponseBody};

use crate::shared::{
    create_test_collection, create_test_component_dir_entry, create_test_endpoint_dir_entry,
    create_test_request_dir_entry, create_test_schema_dir_entry, random_entry_name,
};

// Helper function to scan entries using worktree directly
async fn scan_entries_for_test(
    ctx: &AsyncContext,
    app_delegate: &AppDelegate<MockAppRuntime>,
    collection: &moss_collection::Collection<MockAppRuntime>,
    dir_name: &str,
) -> Vec<StreamEntriesEvent> {
    let entries = Arc::new(Mutex::new(Vec::new()));
    let entries_clone = entries.clone();

    collection
        .stream_entries(
            &ctx,
            app_delegate,
            TauriChannel::new(move |body: InvokeResponseBody| {
                if let InvokeResponseBody::Json(json_str) = body {
                    if let Ok(event) = serde_json::from_str::<StreamEntriesEvent>(&json_str) {
                        entries_clone.lock().unwrap().push(event);
                    }
                }
                Ok(())
            }),
            StreamEntriesInput::ReloadPath(PathBuf::from(dir_name)),
        )
        .await
        .unwrap();

    entries.lock().unwrap().clone()
}

#[tokio::test]
async fn stream_entries_empty_collection() {
    let (ctx, app_delegate, collection_path, collection) = create_test_collection().await;

    // Each directory should return exactly one entry (the directory itself)
    // since the base directories are created with config files
    for dir in &[
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ] {
        let entries = scan_entries_for_test(&ctx, &app_delegate, &collection, dir).await;

        assert_eq!(
            entries.len(),
            1,
            "Expected exactly one entry (the directory itself) in directory {}",
            dir
        );

        // Verify the entry is the directory itself
        let entry = &entries[0];
        assert_eq!(entry.name, *dir);
        assert!(matches!(entry.kind, EntryKind::Dir));
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_single_entry() {
    let (ctx, app_delegate, collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    create_test_request_dir_entry(&ctx, &mut collection, &entry_name).await;

    // Scan the components directory
    let entries = scan_entries_for_test(&ctx, &app_delegate, &collection, dirs::REQUESTS_DIR).await;

    // Should have 2 entries: the directory itself + the created entry
    assert_eq!(
        entries.len(),
        2,
        "Expected two entries: directory + created entry"
    );

    // Find the created entry (not the directory entry)
    let created_entry = entries
        .iter()
        .find(|e| e.name == entry_name)
        .expect("Should find the created entry");

    assert_eq!(created_entry.name, entry_name);
    assert_eq!(
        created_entry
            .path
            .raw
            .file_name()
            .unwrap()
            .to_string_lossy(),
        entry_name
    );

    // Verify the directory entry is also present
    let dir_entry = entries
        .iter()
        .find(|e| e.name == dirs::REQUESTS_DIR)
        .expect("Should find the directory entry");
    assert!(matches!(dir_entry.kind, EntryKind::Dir));

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_multiple_entries_same_directory() {
    let (ctx, app_delegate, collection_path, mut collection) = create_test_collection().await;

    let entry1_name = format!("{}_1", random_entry_name());
    let entry2_name = format!("{}_2", random_entry_name());
    let entry3_name = format!("{}_3", random_entry_name());

    create_test_request_dir_entry(&ctx, &mut collection, &entry1_name).await;
    create_test_request_dir_entry(&ctx, &mut collection, &entry2_name).await;
    create_test_request_dir_entry(&ctx, &mut collection, &entry3_name).await;

    let entries = scan_entries_for_test(&ctx, &app_delegate, &collection, dirs::REQUESTS_DIR).await;

    // Should have 4 entries: the directory itself + 3 created entries
    assert_eq!(
        entries.len(),
        4,
        "Expected four entries: directory + 3 created entries"
    );

    // Filter out the directory entry to check created entries
    let created_entries: Vec<_> = entries
        .iter()
        .filter(|e| e.name != dirs::REQUESTS_DIR)
        .collect();
    assert_eq!(
        created_entries.len(),
        3,
        "Expected exactly three created entries"
    );

    let entry_names: Vec<&str> = created_entries.iter().map(|e| e.name.as_str()).collect();
    assert!(entry_names.contains(&entry1_name.as_str()));
    assert!(entry_names.contains(&entry2_name.as_str()));
    assert!(entry_names.contains(&entry3_name.as_str()));

    // Verify the directory entry is present
    let dir_entry = entries
        .iter()
        .find(|e| e.name == dirs::REQUESTS_DIR)
        .expect("Should find the directory entry");
    assert!(matches!(dir_entry.kind, EntryKind::Dir));

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_multiple_directories() {
    let (ctx, app_delegate, collection_path, mut collection) = create_test_collection().await;

    let expected_name = "entry".to_string();

    // We have to manually do this now, since we will validate path against configuration
    let _ = create_test_request_dir_entry(&ctx, &mut collection, &expected_name).await;
    let _ = create_test_endpoint_dir_entry(&ctx, &mut collection, &expected_name).await;
    let _ = create_test_component_dir_entry(&ctx, &mut collection, &expected_name).await;
    let _ = create_test_schema_dir_entry(&ctx, &mut collection, &expected_name).await;

    let directories = [
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ];

    for dir in directories {
        let entries = scan_entries_for_test(&ctx, &app_delegate, &collection, dir).await;

        // Should have 2 entries: the directory itself + the created entry
        assert_eq!(
            entries.len(),
            2,
            "Expected two entries: directory + created entry in {}",
            dir
        );

        // Find the created entry (not the directory)
        let created_entry = entries
            .iter()
            .find(|e| e.name == expected_name)
            .expect(&format!("Should find created entry in {}", dir));

        assert_eq!(created_entry.name, expected_name);

        // Verify the directory entry is present
        let dir_entry = entries
            .iter()
            .find(|e| e.name == dir)
            .expect(&format!("Should find directory entry for {}", dir));
        assert!(matches!(dir_entry.kind, EntryKind::Dir));
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_scan_operation_stability() {
    let (ctx, app_delegate, collection_path, mut collection) = create_test_collection().await;

    // Create some entries to test scan stability
    let entry1_name = format!("{}_1", random_entry_name());
    let entry2_name = format!("{}_2", random_entry_name());

    let _ = create_test_component_dir_entry(&ctx, &mut collection, &entry1_name).await;
    let _ = create_test_request_dir_entry(&ctx, &mut collection, &entry2_name).await;

    // Test that scan operations don't crash with mixed content
    for dir in &[
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ] {
        let entries = scan_entries_for_test(&ctx, &app_delegate, &collection, dir).await;
        // Each directory should have at least 1 entry (the directory itself)
        // and at most 2 entries (directory + 1 created entry)
        assert!(
            entries.len() >= 1 && entries.len() <= 2,
            "Expected 1-2 entries in directory {}, found {}",
            dir,
            entries.len()
        );
    }

    // Test concurrent scans (simulating what stream_entries does)
    let futures = [
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ]
    .iter()
    .map(|dir| scan_entries_for_test(&ctx, &app_delegate, &collection, dir));

    let results = futures::future::join_all(futures).await;

    // Verify all scans completed successfully
    assert_eq!(results.len(), 4, "All 4 directory scans should complete");

    // Total entries should be: 4 directories + 2 created entries = 6
    let total_entries: usize = results.iter().map(|r| r.len()).sum();
    assert_eq!(
        total_entries, 6,
        "Should find exactly 6 entries total (4 directories + 2 created)"
    );

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_mixed_content() {
    let (ctx, app_delegate, collection_path, mut collection) = create_test_collection().await;

    // Create entries in multiple directories
    let requests_entry = format!("{}_requests", random_entry_name());
    let components_entry1 = format!("{}_comp1", random_entry_name());
    let components_entry2 = format!("{}_comp2", random_entry_name());
    let schemas_entry = format!("{}_schema", random_entry_name());

    create_test_request_dir_entry(&ctx, &mut collection, &requests_entry).await;
    create_test_component_dir_entry(&ctx, &mut collection, &components_entry1).await;
    create_test_component_dir_entry(&ctx, &mut collection, &components_entry2).await;
    create_test_schema_dir_entry(&ctx, &mut collection, &schemas_entry).await;

    // Test each directory independently

    // Requests should have 2 entries: directory + 1 created entry
    let requests_entries =
        scan_entries_for_test(&ctx, &app_delegate, &collection, dirs::REQUESTS_DIR).await;
    assert_eq!(requests_entries.len(), 2);
    let created_request = requests_entries
        .iter()
        .find(|e| e.name == requests_entry)
        .expect("Should find created request entry");
    assert_eq!(created_request.name, requests_entry);

    // Components should have 3 entries: directory + 2 created entries
    let components_entries =
        scan_entries_for_test(&ctx, &app_delegate, &collection, dirs::COMPONENTS_DIR).await;
    assert_eq!(components_entries.len(), 3);
    let created_components: Vec<_> = components_entries
        .iter()
        .filter(|e| e.name != dirs::COMPONENTS_DIR)
        .collect();
    assert_eq!(created_components.len(), 2);
    let component_names: Vec<&str> = created_components.iter().map(|e| e.name.as_str()).collect();
    assert!(component_names.contains(&components_entry1.as_str()));
    assert!(component_names.contains(&components_entry2.as_str()));

    // Schemas should have 2 entries: directory + 1 created entry
    let schemas_entries =
        scan_entries_for_test(&ctx, &app_delegate, &collection, dirs::SCHEMAS_DIR).await;
    assert_eq!(schemas_entries.len(), 2);
    let created_schema = schemas_entries
        .iter()
        .find(|e| e.name == schemas_entry)
        .expect("Should find created schema entry");
    assert_eq!(created_schema.name, schemas_entry);

    // Endpoints should have 1 entry: just the directory
    let endpoints_entries =
        scan_entries_for_test(&ctx, &app_delegate, &collection, dirs::ENDPOINTS_DIR).await;
    assert_eq!(endpoints_entries.len(), 1);
    let dir_entry = endpoints_entries
        .iter()
        .find(|e| e.name == dirs::ENDPOINTS_DIR)
        .expect("Should find directory entry");
    assert!(matches!(dir_entry.kind, EntryKind::Dir));

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_verify_entry_properties() {
    let (ctx, app_delegate, collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    create_test_request_dir_entry(&ctx, &mut collection, &entry_name).await;

    let entries = scan_entries_for_test(&ctx, &app_delegate, &collection, dirs::REQUESTS_DIR).await;
    // Should have 2 entries: directory + created entry
    assert_eq!(entries.len(), 2);

    // Find the created entry (not the directory)
    let created_entry = entries
        .iter()
        .find(|e| e.name == entry_name)
        .expect("Should find the created entry");

    // Verify all properties are set correctly for the created entry
    assert_eq!(created_entry.name, entry_name, "Name should match");
    assert!(
        !created_entry.path.raw.to_string_lossy().is_empty(),
        "Path should be set"
    );

    // The entry should be a directory entry with request classification
    assert!(
        matches!(
            created_entry.kind,
            moss_collection::models::primitives::EntryKind::Dir
        ),
        "Should be a directory"
    );
    assert!(
        matches!(
            created_entry.class,
            moss_collection::models::primitives::EntryClass::Request
        ),
        "Should be classified as Request"
    );

    // Verify the directory entry is also present
    let dir_entry = entries
        .iter()
        .find(|e| e.name == dirs::REQUESTS_DIR)
        .expect("Should find the directory entry");
    assert!(matches!(dir_entry.kind, EntryKind::Dir));

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}
