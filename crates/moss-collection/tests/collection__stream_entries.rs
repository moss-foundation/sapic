pub mod shared;

use futures;
use moss_collection::{
    dirs,
    models::{
        operations::{CreateDirEntryInput, CreateEntryInput},
        primitives::EntryKind,
        types::configuration::{
            DirConfigurationModel, DirHttpConfigurationModel, DirRequestConfigurationModel,
        },
    },
    services::worktree_service::EntryDescription,
};
use moss_testutils::random_name::random_string;
use std::path::PathBuf;
use tokio::sync::mpsc;

use crate::shared::create_test_collection;

fn random_entry_name() -> String {
    format!("Test_{}_Entry", random_string(10))
}

fn create_test_dir_configuration() -> DirConfigurationModel {
    DirConfigurationModel::Request(DirRequestConfigurationModel::Http(
        DirHttpConfigurationModel {},
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
        order: 0,
        configuration: create_test_dir_configuration(),
    });

    collection.create_entry(input).await.unwrap();
}

// Helper function to scan entries using worktree directly
// This bypasses the Tauri Channel requirement for testing
async fn scan_entries_for_test(
    collection: &moss_collection::Collection,
    dir_name: &str,
) -> Vec<EntryDescription> {
    let (tx, mut rx) = mpsc::unbounded_channel::<EntryDescription>();

    // Access the worktree service through the Collection's service system
    let worktree_service =
        collection.service::<moss_collection::services::worktree_service::WorktreeService>();

    let dir_path = std::path::Path::new(dir_name);

    // Check if directory exists before scanning
    let abs_dir = worktree_service.absolutize(dir_path);
    if abs_dir.is_err() || !abs_dir.as_ref().unwrap().exists() {
        return Vec::new(); // Return empty vec if directory doesn't exist
    }

    // Create empty sets for scanning
    let expanded_entries = std::sync::Arc::new(std::collections::HashSet::new());
    let all_entry_keys = std::sync::Arc::new(std::collections::HashMap::new());

    let _result = worktree_service
        .scan(dir_path, expanded_entries, all_entry_keys, tx)
        .await;

    let mut entries = Vec::new();
    while let Ok(entry) = rx.try_recv() {
        entries.push(entry);
    }

    entries
}

#[tokio::test]
async fn stream_entries_empty_collection() {
    let (collection_path, collection) = create_test_collection().await;

    // Each directory should return exactly one entry (the directory itself)
    // since the base directories are created with config files
    for dir in &[
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
    ] {
        let entries = scan_entries_for_test(&collection, dir).await;
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
    let (collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    create_test_entry_in_dir(&mut collection, &entry_name, dirs::COMPONENTS_DIR).await;

    // Scan the components directory
    let entries = scan_entries_for_test(&collection, dirs::COMPONENTS_DIR).await;

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
        created_entry.path.file_name().unwrap().to_string_lossy(),
        entry_name
    );
    assert!(!created_entry.id.is_nil());

    // Verify the directory entry is also present
    let dir_entry = entries
        .iter()
        .find(|e| e.name == dirs::COMPONENTS_DIR)
        .expect("Should find the directory entry");
    assert!(matches!(dir_entry.kind, EntryKind::Dir));

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

    let entries = scan_entries_for_test(&collection, dirs::REQUESTS_DIR).await;

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

    // Verify all entries have valid IDs
    for entry in &created_entries {
        assert!(
            !entry.id.is_nil(),
            "Entry {} should have a valid ID",
            entry.name
        );
    }

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
            .find(|e| e.name == *expected_name)
            .expect(&format!(
                "Should find created entry {} in {}",
                expected_name, dir
            ));

        assert_eq!(created_entry.name, *expected_name);
        assert!(!created_entry.id.is_nil());

        // Verify the directory entry is present
        let dir_entry = entries
            .iter()
            .find(|e| e.name == **dir)
            .expect(&format!("Should find directory entry for {}", dir));
        assert!(matches!(dir_entry.kind, EntryKind::Dir));
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
    .map(|dir| scan_entries_for_test(&collection, dir));

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

    // Requests should have 2 entries: directory + 1 created entry
    let requests_entries = scan_entries_for_test(&collection, dirs::REQUESTS_DIR).await;
    assert_eq!(requests_entries.len(), 2);
    let created_request = requests_entries
        .iter()
        .find(|e| e.name == requests_entry)
        .expect("Should find created request entry");
    assert_eq!(created_request.name, requests_entry);

    // Components should have 3 entries: directory + 2 created entries
    let components_entries = scan_entries_for_test(&collection, dirs::COMPONENTS_DIR).await;
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
    let schemas_entries = scan_entries_for_test(&collection, dirs::SCHEMAS_DIR).await;
    assert_eq!(schemas_entries.len(), 2);
    let created_schema = schemas_entries
        .iter()
        .find(|e| e.name == schemas_entry)
        .expect("Should find created schema entry");
    assert_eq!(created_schema.name, schemas_entry);

    // Endpoints should have 1 entry: just the directory
    let endpoints_entries = scan_entries_for_test(&collection, dirs::ENDPOINTS_DIR).await;
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
    let (collection_path, mut collection) = create_test_collection().await;

    let entry_name = random_entry_name();
    create_test_entry_in_dir(&mut collection, &entry_name, dirs::COMPONENTS_DIR).await;

    let entries = scan_entries_for_test(&collection, dirs::COMPONENTS_DIR).await;
    // Should have 2 entries: directory + created entry
    assert_eq!(entries.len(), 2);

    // Find the created entry (not the directory)
    let created_entry = entries
        .iter()
        .find(|e| e.name == entry_name)
        .expect("Should find the created entry");

    // Verify all properties are set correctly for the created entry
    assert!(!created_entry.id.is_nil(), "ID should be set");
    assert_eq!(created_entry.name, entry_name, "Name should match");
    assert!(
        !created_entry.path.to_string_lossy().is_empty(),
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
        .find(|e| e.name == dirs::COMPONENTS_DIR)
        .expect("Should find the directory entry");
    assert!(matches!(dir_entry.kind, EntryKind::Dir));

    // Cleanup
    std::fs::remove_dir_all(collection_path).unwrap();
}
