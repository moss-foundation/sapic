#![cfg(feature = "integration-tests")]

use sapic_ipc::contracts::main::{
    project::{CreateProjectInput, CreateProjectParams},
    resource::{ListProjectResourcesInput, ListProjectResourcesMode},
};

use crate::shared::{
    create_test_component_dir_entry, create_test_endpoint_dir_entry, create_test_schema_dir_entry,
    random_entry_name, set_up_test_main_window,
};

mod shared;

#[tokio::test]
async fn list_project_resources_empty_project() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "Test Project".to_string(),
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    let output = main_window
        .list_project_resources(
            &ctx,
            ListProjectResourcesInput {
                project_id: project_id.clone(),
                mode: ListProjectResourcesMode::LoadRoot,
            },
        )
        .await
        .unwrap();

    assert_eq!(output.items.len(), 0);

    cleanup().await;
}

#[tokio::test]
async fn list_project_resources_single_entry() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "Test Project".to_string(),
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    let entry_name = random_entry_name();
    create_test_endpoint_dir_entry(&main_window, &ctx, &project_id, &entry_name).await;

    let output = main_window
        .list_project_resources(
            &ctx,
            ListProjectResourcesInput {
                project_id: project_id.clone(),
                mode: ListProjectResourcesMode::LoadRoot,
            },
        )
        .await
        .unwrap();

    assert_eq!(output.items.len(), 1);

    let created_entry = output
        .items
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

    cleanup().await;
}

#[tokio::test]
async fn list_project_resources_multiple_entries_same_directory() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "Test Project".to_string(),
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    let entry1_name = format!("{}_1", random_entry_name());
    let entry2_name = format!("{}_2", random_entry_name());
    let entry3_name = format!("{}_3", random_entry_name());

    create_test_endpoint_dir_entry(&main_window, &ctx, &project_id, &entry1_name).await;
    create_test_endpoint_dir_entry(&main_window, &ctx, &project_id, &entry2_name).await;
    create_test_endpoint_dir_entry(&main_window, &ctx, &project_id, &entry3_name).await;

    let output = main_window
        .list_project_resources(
            &ctx,
            ListProjectResourcesInput {
                project_id: project_id.clone(),
                mode: ListProjectResourcesMode::LoadRoot,
            },
        )
        .await
        .unwrap();

    assert_eq!(
        output.items.len(),
        3,
        "Expected three entries: 3 created entries"
    );

    let entry_names: Vec<&str> = output.items.iter().map(|e| e.name.as_str()).collect();
    assert!(entry_names.contains(&entry1_name.as_str()));
    assert!(entry_names.contains(&entry2_name.as_str()));
    assert!(entry_names.contains(&entry3_name.as_str()));

    cleanup().await;
}

#[tokio::test]
async fn list_project_resources_multiple_directories() {
    let (main_window, _delegate, ctx, cleanup, _) = set_up_test_main_window().await;

    let project_id = main_window
        .create_project(
            &ctx,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: "Test Project".to_string(),
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    let mut entry_names = Vec::new();
    {
        let name = random_entry_name();
        create_test_endpoint_dir_entry(&main_window, &ctx, &project_id, &name).await;
        entry_names.push(name);
    }
    {
        let name = random_entry_name();
        create_test_component_dir_entry(&main_window, &ctx, &project_id, &name).await;
        entry_names.push(name);
    }
    {
        let name = random_entry_name();
        create_test_schema_dir_entry(&main_window, &ctx, &project_id, &name).await;
        entry_names.push(name);
    }

    for name in &entry_names {
        let output = main_window
            .list_project_resources(
                &ctx,
                ListProjectResourcesInput {
                    project_id: project_id.clone(),
                    mode: ListProjectResourcesMode::ReloadPath(std::path::PathBuf::from(name)),
                },
            )
            .await
            .unwrap();

        assert_eq!(output.items.len(), 1);

        let created_entry = output
            .items
            .iter()
            .find(|e| e.name == *name)
            .unwrap_or_else(|| panic!("Should find created entry in {}", name));

        assert_eq!(created_entry.name, *name);
    }

    cleanup().await;
}
