#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_app_delegate::AppDelegate;
use moss_applib::mock::MockAppRuntime;
use moss_project::{
    dirs,
    models::{events::StreamResourcesEvent, operations::StreamResourcesInput},
};
use sapic_core::context::AsyncContext;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::ipc::{Channel as TauriChannel, InvokeResponseBody};

use crate::shared::{
    RESOURCES_ROOT_DIR, create_test_component_dir_entry, create_test_endpoint_dir_entry,
    create_test_project, create_test_schema_dir_entry, random_entry_name,
};

// Helper function to scan entries using worktree directly
async fn scan_entries_for_test(
    ctx: &AsyncContext,
    app_delegate: &AppDelegate<MockAppRuntime>,
    project: &moss_project::Project<MockAppRuntime>,
    dir_name: &str,
) -> Vec<StreamResourcesEvent> {
    let entries = Arc::new(Mutex::new(Vec::new()));
    let entries_clone = entries.clone();

    project
        .stream_resources(
            &ctx,
            app_delegate,
            TauriChannel::new(move |body: InvokeResponseBody| {
                if let InvokeResponseBody::Json(json_str) = body {
                    if let Ok(event) = serde_json::from_str::<StreamResourcesEvent>(&json_str) {
                        entries_clone.lock().unwrap().push(event);
                    }
                }
                Ok(())
            }),
            StreamResourcesInput::ReloadPath(PathBuf::from(dir_name)),
        )
        .await
        .unwrap();

    entries.lock().unwrap().clone()
}

#[tokio::test]
async fn stream_entries_empty_project() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entries = scan_entries_for_test(&ctx, &app_delegate, &project, dirs::RESOURCES_DIR).await;

    assert_eq!(entries.len(), 0);

    // Cleanup
    cleanup();
}

#[tokio::test]
async fn stream_entries_single_entry() {
    let (ctx, app_delegate, _, mut project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    create_test_endpoint_dir_entry(&ctx, &mut project, &entry_name).await;

    // Scan the resources directory
    let entries = scan_entries_for_test(&ctx, &app_delegate, &project, RESOURCES_ROOT_DIR).await;

    assert_eq!(entries.len(), 1);

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

    // Cleanup
    cleanup();
}

#[tokio::test]
async fn stream_entries_multiple_entries_same_directory() {
    let (ctx, app_delegate, _, mut project, cleanup) = create_test_project().await;

    let entry1_name = format!("{}_1", random_entry_name());
    let entry2_name = format!("{}_2", random_entry_name());
    let entry3_name = format!("{}_3", random_entry_name());

    create_test_endpoint_dir_entry(&ctx, &mut project, &entry1_name).await;
    create_test_endpoint_dir_entry(&ctx, &mut project, &entry2_name).await;
    create_test_endpoint_dir_entry(&ctx, &mut project, &entry3_name).await;

    let entries = scan_entries_for_test(&ctx, &app_delegate, &project, RESOURCES_ROOT_DIR).await;

    assert_eq!(
        entries.len(),
        3,
        "Expected three entries: 3 created entries"
    );

    let entry_names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert!(entry_names.contains(&entry1_name.as_str()));
    assert!(entry_names.contains(&entry2_name.as_str()));
    assert!(entry_names.contains(&entry3_name.as_str()));

    // Cleanup
    cleanup();
}

#[tokio::test]
async fn stream_entries_multiple_directories() {
    let (ctx, app_delegate, _, mut project, cleanup) = create_test_project().await;

    let mut entries = Vec::new();
    {
        let name = random_entry_name();
        let id = create_test_endpoint_dir_entry(&ctx, &mut project, &name).await;
        entries.push((id, name));
    }

    {
        let name = random_entry_name();
        let id = create_test_component_dir_entry(&ctx, &mut project, &name).await;
        entries.push((id, name));
    }

    {
        let name = random_entry_name();
        let id = create_test_schema_dir_entry(&ctx, &mut project, &name).await;
        entries.push((id, name));
    }

    for (_, name) in entries {
        let scan_result = scan_entries_for_test(&ctx, &app_delegate, &project, &name).await;

        // Should have 2 entries: the directory itself + the created entry
        assert_eq!(scan_result.len(), 1,);

        // Find the created entry (not the directory)
        let created_entry = scan_result
            .iter()
            .find(|e| e.name == name)
            .expect(&format!("Should find created entry in {}", name));

        assert_eq!(created_entry.name, name);
    }

    // Cleanup
    cleanup();
}
