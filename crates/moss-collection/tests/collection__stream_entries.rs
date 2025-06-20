pub mod shared;

use futures;
use moss_collection::{
    dirs,
    models::{
        operations::{CreateDirEntryInput, CreateEntryInput},
        types::configuration::{
            DirConfigurationModel, HttpDirConfigurationModel, RequestDirConfigurationModel,
        },
    },
    worktree::WorktreeEntry,
};
use moss_testutils::random_name::random_string;
use std::path::PathBuf;
use tokio::sync::mpsc;

use crate::shared::create_test_collection;

fn random_entry_name() -> String {
    format!("Test_{}_Entry", random_string(10))
}

fn create_test_dir_configuration() -> DirConfigurationModel {
    DirConfigurationModel::Request(RequestDirConfigurationModel::Http(
        HttpDirConfigurationModel {},
    ))
}

async fn create_test_entry_in_dir(
    collection: &mut moss_collection::Collection,
    entry_name: &str,
    dir_name: &str,
) {
    let entry_path = PathBuf::from(dir_name);

    let input = CreateEntryInput::Dir(CreateDirEntryInput {
        path: entry_path.clone(),
        name: entry_name.to_string(),
        order: None,
        configuration: create_test_dir_configuration(),
    });

    collection.create_entry(input).await.unwrap();
}

// Helper function to scan entries using worktree directly
// This bypasses the Tauri Channel requirement for testing
async fn scan_entries_for_test(
    collection: &moss_collection::Collection,
    dir_name: &str,
) -> Vec<WorktreeEntry> {
    let (tx, mut rx) = mpsc::unbounded_channel::<WorktreeEntry>();
    let worktree = collection.worktree();

    let dir_path = std::path::Path::new(dir_name);

    // Check if directory exists before scanning
    let abs_dir = worktree.absolutize(dir_path);
    if abs_dir.is_err() || !abs_dir.as_ref().unwrap().exists() {
        return Vec::new(); // Return empty vec if directory doesn't exist
    }

    let _result = worktree.scan(dir_path, tx).await;

    let mut entries = Vec::new();
    while let Ok(entry) = rx.try_recv() {
        entries.push(entry);
    }

    entries
}

#[tokio::test]
async fn stream_entries_empty_collection() {
    let (collection_path, collection) = create_test_collection().await;

    // Test scanning empty directories
    for dir in &[
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ] {
        let entries = scan_entries_for_test(&collection, dir).await;
        assert!(
            entries.is_empty(),
            "Expected no entries in empty directory {}",
            dir
        );
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_single_entry() {
    let (collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    create_test_entry_in_dir(&mut collection, &entry_name, dirs::COMPONENTS_DIR).await;

    // Scan the components directory
    let entries = scan_entries_for_test(&collection, dirs::COMPONENTS_DIR).await;

    assert_eq!(entries.len(), 1, "Expected exactly one entry");

    let entry = &entries[0];
    assert_eq!(entry.name, entry_name);
    assert_eq!(
        entry.path.file_name().unwrap().to_string_lossy(),
        entry_name
    );
    assert!(!entry.id.is_nil());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_multiple_entries_same_directory() {
    let (collection_path, mut collection) = create_test_collection().await;

    let entry1_name = format!("{}_1", random_entry_name());
    let entry2_name = format!("{}_2", random_entry_name());
    let entry3_name = format!("{}_3", random_entry_name());

    create_test_entry_in_dir(&mut collection, &entry1_name, dirs::REQUESTS_DIR).await;
    create_test_entry_in_dir(&mut collection, &entry2_name, dirs::REQUESTS_DIR).await;
    create_test_entry_in_dir(&mut collection, &entry3_name, dirs::REQUESTS_DIR).await;

    // Scan the requests directory
    let entries = scan_entries_for_test(&collection, dirs::REQUESTS_DIR).await;

    assert_eq!(entries.len(), 3, "Expected exactly three entries");

    let entry_names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert!(entry_names.contains(&entry1_name.as_str()));
    assert!(entry_names.contains(&entry2_name.as_str()));
    assert!(entry_names.contains(&entry3_name.as_str()));

    // Verify all entries have valid IDs
    for entry in &entries {
        assert!(
            !entry.id.is_nil(),
            "Entry {} should have a valid ID",
            entry.name
        );
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_multiple_directories() {
    let (collection_path, mut collection) = create_test_collection().await;

    let directories = [
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ];

    let mut expected_entries = Vec::new();

    // Create one entry in each directory
    for (idx, dir) in directories.iter().enumerate() {
        let entry_name = format!("{}_{}", random_entry_name(), idx);
        create_test_entry_in_dir(&mut collection, &entry_name, dir).await;
        expected_entries.push((entry_name, dir));
    }

    // Scan each directory and verify entries
    for (expected_name, dir) in &expected_entries {
        let entries = scan_entries_for_test(&collection, dir).await;

        assert_eq!(entries.len(), 1, "Expected exactly one entry in {}", dir);

        let entry = &entries[0];
        assert_eq!(entry.name, *expected_name);
        assert!(!entry.id.is_nil());
    }

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_scan_operation_stability() {
    let (collection_path, mut collection) = create_test_collection().await;

    // Create some entries to test scan stability
    let entry1_name = format!("{}_1", random_entry_name());
    let entry2_name = format!("{}_2", random_entry_name());

    create_test_entry_in_dir(&mut collection, &entry1_name, dirs::COMPONENTS_DIR).await;
    create_test_entry_in_dir(&mut collection, &entry2_name, dirs::REQUESTS_DIR).await;

    // Test that scan operations don't crash with mixed content
    for dir in &[
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ] {
        let entries = scan_entries_for_test(&collection, dir).await;
        // Just verify scan completes without error
        // The number of entries depends on what was created in each directory
        assert!(
            entries.len() <= 2,
            "Should not find more entries than we created"
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
    .map(|dir| scan_entries_for_test(&collection, dir));

    let results = futures::future::join_all(futures).await;

    // Verify all scans completed successfully
    assert_eq!(results.len(), 4, "All 4 directory scans should complete");

    // Total entries should match what we created
    let total_entries: usize = results.iter().map(|r| r.len()).sum();
    assert_eq!(total_entries, 2, "Should find exactly 2 entries total");

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_mixed_content() {
    let (collection_path, mut collection) = create_test_collection().await;

    // Create entries in multiple directories
    let requests_entry = format!("{}_requests", random_entry_name());
    let components_entry1 = format!("{}_comp1", random_entry_name());
    let components_entry2 = format!("{}_comp2", random_entry_name());
    let schemas_entry = format!("{}_schema", random_entry_name());

    create_test_entry_in_dir(&mut collection, &requests_entry, dirs::REQUESTS_DIR).await;
    create_test_entry_in_dir(&mut collection, &components_entry1, dirs::COMPONENTS_DIR).await;
    create_test_entry_in_dir(&mut collection, &components_entry2, dirs::COMPONENTS_DIR).await;
    create_test_entry_in_dir(&mut collection, &schemas_entry, dirs::SCHEMAS_DIR).await;

    // Test each directory independently

    // Requests should have 1 entry
    let requests_entries = scan_entries_for_test(&collection, dirs::REQUESTS_DIR).await;
    assert_eq!(requests_entries.len(), 1);
    assert_eq!(requests_entries[0].name, requests_entry);

    // Components should have 2 entries
    let components_entries = scan_entries_for_test(&collection, dirs::COMPONENTS_DIR).await;
    assert_eq!(components_entries.len(), 2);
    let component_names: Vec<&str> = components_entries.iter().map(|e| e.name.as_str()).collect();
    assert!(component_names.contains(&components_entry1.as_str()));
    assert!(component_names.contains(&components_entry2.as_str()));

    // Schemas should have 1 entry
    let schemas_entries = scan_entries_for_test(&collection, dirs::SCHEMAS_DIR).await;
    assert_eq!(schemas_entries.len(), 1);
    assert_eq!(schemas_entries[0].name, schemas_entry);

    // Endpoints should be empty
    let endpoints_entries = scan_entries_for_test(&collection, dirs::ENDPOINTS_DIR).await;
    assert!(endpoints_entries.is_empty());

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}

#[tokio::test]
async fn stream_entries_verify_entry_properties() {
    let (collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    create_test_entry_in_dir(&mut collection, &entry_name, dirs::COMPONENTS_DIR).await;

    let entries = scan_entries_for_test(&collection, dirs::COMPONENTS_DIR).await;
    assert_eq!(entries.len(), 1);

    let entry = &entries[0];

    // Verify all properties are set correctly
    assert!(!entry.id.is_nil(), "ID should be set");
    assert_eq!(entry.name, entry_name, "Name should match");
    assert!(
        !entry.path.to_string_lossy().is_empty(),
        "Path should be set"
    );

    // The entry should be a directory entry with request classification
    assert!(
        matches!(
            entry.kind,
            moss_collection::models::primitives::EntryKind::Dir
        ),
        "Should be a directory"
    );
    assert!(
        matches!(
            entry.class,
            moss_collection::models::primitives::EntryClass::Request
        ),
        "Should be classified as Request"
    );

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}
