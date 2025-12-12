#![cfg(feature = "integration-tests")]

pub mod shared;

use crate::shared::{generate_random_icon, setup_test_workspace};
use moss_applib::mock::MockAppRuntime;
use moss_storage2::models::primitives::StorageScope;
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_project_name};
use moss_workspace::{
    models::{operations::CreateProjectInput, types::CreateProjectParams},
    storage::{KEY_EXPANDED_ITEMS, key_project_order},
};
use sapic_base::project::types::primitives::ProjectId;
use sapic_runtime::globals::GlobalKvStorage;
use serde_json::Value as JsonValue;
use std::{collections::HashSet, path::Path};
use tauri::ipc::Channel;

#[tokio::test]
async fn create_project_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = random_project_name();
    let create_project_output = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    // Verify through stream_projects
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_projects::<MockAppRuntime>(&ctx, channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the directory was created
    assert!(create_project_output.abs_path.exists());

    let id = create_project_output.id;
    // Verify the db entries were created
    let storage = GlobalKvStorage::get(&app_delegate);
    // Check order was stored
    let order_value = storage
        .get(
            &ctx,
            StorageScope::Workspace(workspace.id().inner()),
            &key_project_order(&id),
        )
        .await
        .unwrap()
        .unwrap();
    let order: isize = serde_json::from_value(order_value).unwrap();

    assert_eq!(order, 0);
    // Check expanded_items contains the project id
    let expanded_items_value = storage
        .get(
            &ctx,
            StorageScope::Workspace(workspace.id().inner()),
            KEY_EXPANDED_ITEMS,
        )
        .await
        .unwrap()
        .unwrap();
    let expanded_items: HashSet<ProjectId> = serde_json::from_value(expanded_items_value).unwrap();
    assert!(expanded_items.contains(&id));

    cleanup().await;
}

#[tokio::test]
async fn create_project_empty_name() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = "".to_string();
    let create_project_result = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await;

    assert!(create_project_result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn create_project_special_chars() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name_list = FILENAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_project_name()))
        .collect::<Vec<String>>();

    for project_name in &project_name_list {
        let create_project_result = workspace
            .create_project(
                &ctx,
                &app_delegate,
                &CreateProjectInput {
                    inner: CreateProjectParams {
                        name: project_name.clone(),
                        order: 0,
                        external_path: None,
                        git_params: None,
                        icon_path: None,
                    },
                },
            )
            .await;

        let create_project_output = create_project_result.unwrap();

        // Verify the directory was created
        assert!(create_project_output.abs_path.exists());

        let id = create_project_output.id;

        // Verify the db entries were created
        let storage = GlobalKvStorage::get(&app_delegate);
        // Check order was stored
        let order_value = storage
            .get(
                &ctx,
                StorageScope::Workspace(workspace.id().inner()),
                &key_project_order(&id),
            )
            .await
            .unwrap()
            .unwrap();
        let order: isize = serde_json::from_value(order_value).unwrap();

        assert_eq!(order, 0);
        // Check expanded_items contains the project id
        let expanded_items_value = storage
            .get(
                &ctx,
                StorageScope::Workspace(workspace.id().inner()),
                KEY_EXPANDED_ITEMS,
            )
            .await
            .unwrap()
            .unwrap();
        let expanded_items: HashSet<ProjectId> =
            serde_json::from_value(expanded_items_value).unwrap();
        assert!(expanded_items.contains(&id));
    }

    // Verify all projects are returned through stream_projects
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_projects::<MockAppRuntime>(&ctx, channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, project_name_list.len());

    cleanup().await;
}

#[tokio::test]
async fn create_project_with_order() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = random_project_name();
    let create_project_result = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: 42,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await;

    let create_project_output = create_project_result.unwrap();

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_projects::<MockAppRuntime>(&ctx, channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the directory was created
    assert!(create_project_output.abs_path.exists());

    let id = create_project_output.id;

    // Verify the db entries were created
    let storage = GlobalKvStorage::get(&app_delegate);
    // Check order was stored
    let order_value = storage
        .get(
            &ctx,
            StorageScope::Workspace(workspace.id().inner()),
            &key_project_order(&id),
        )
        .await
        .unwrap()
        .unwrap();
    let order: isize = serde_json::from_value(order_value).unwrap();

    assert_eq!(order, 42);
    // Check expanded_items contains the project id
    let expanded_items_value = storage
        .get(
            &ctx,
            StorageScope::Workspace(workspace.id().inner()),
            KEY_EXPANDED_ITEMS,
        )
        .await
        .unwrap()
        .unwrap();
    let expanded_items: HashSet<ProjectId> = serde_json::from_value(expanded_items_value).unwrap();
    assert!(expanded_items.contains(&id));

    cleanup().await;
}

#[tokio::test]
async fn create_project_with_icon() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = random_project_name();
    let input_icon_path = workspace.abs_path().join("test_icon.png");
    generate_random_icon(&input_icon_path);

    let create_project_result = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: Some(input_icon_path.clone()),
                },
            },
        )
        .await;

    let create_project_output = create_project_result.unwrap();

    let id = create_project_output.id;

    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_projects::<MockAppRuntime>(&ctx, channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 1);

    // Verify the directory was created
    let project_path = create_project_output.abs_path;
    assert!(project_path.exists());

    // Verify that the icon is stored in the assets folder
    let project = workspace.project(&id).await.unwrap();
    assert!(project.icon_path().is_some());

    // Verify the db entries were created
    let storage = GlobalKvStorage::get(&app_delegate);
    // Check order was stored
    let order_value = storage
        .get(
            &ctx,
            StorageScope::Workspace(workspace.id().inner()),
            &key_project_order(&id),
        )
        .await
        .unwrap()
        .unwrap();
    let order: isize = serde_json::from_value(order_value).unwrap();

    assert_eq!(order, 0);
    // Check expanded_items contains the project id
    let expanded_items_value = storage
        .get(
            &ctx,
            StorageScope::Workspace(workspace.id().inner()),
            KEY_EXPANDED_ITEMS,
        )
        .await
        .unwrap()
        .unwrap();
    let expanded_items: HashSet<ProjectId> = serde_json::from_value(expanded_items_value).unwrap();
    assert!(expanded_items.contains(&id));

    cleanup().await;
}

#[tokio::test]
async fn create_multiple_projects_expanded_items() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    // Create first project
    let project_name1 = random_project_name();
    let create_result1 = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name1.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    // Create second project
    let project_name2 = random_project_name();
    let create_result2 = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name2.clone(),
                    order: 1,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    // Check expanded_items contains both project ids
    let storage = GlobalKvStorage::get(&app_delegate);
    // Check expanded_items contains the project id
    let expanded_items_value = storage
        .get(
            &ctx,
            StorageScope::Workspace(workspace.id().inner()),
            KEY_EXPANDED_ITEMS,
        )
        .await
        .unwrap()
        .unwrap();
    let expanded_items: HashSet<ProjectId> = serde_json::from_value(expanded_items_value).unwrap();

    assert_eq!(expanded_items.len(), 2);
    assert!(expanded_items.contains(&create_result1.id));
    assert!(expanded_items.contains(&create_result2.id));

    cleanup().await;
}

#[tokio::test]
async fn create_project_external_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let project_name = random_project_name();
    let external_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("external_projects")
        .join(&project_name);
    tokio::fs::create_dir_all(&external_path).await.unwrap();

    let create_project_output = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: 0,
                    external_path: Some(external_path.clone()),
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    assert_eq!(
        create_project_output.external_path,
        Some(external_path.clone())
    );

    // Verify the internal directory is created
    assert!(create_project_output.abs_path.exists());

    let internal_path = create_project_output.abs_path;
    assert!(internal_path.exists());

    // Verify the correct config is created in internal directory
    let config_path = internal_path.join("config.json");
    assert!(config_path.exists());
    let config: JsonValue =
        serde_json::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap();
    assert_eq!(
        Path::new(config["external_path"].as_str().unwrap()),
        external_path
    );

    // Verify the manifest is created in external directory
    let manifest_path = external_path.join("Sapic.json");
    assert!(manifest_path.exists());

    // Verify through stream_projects
    let channel = Channel::new(move |_| Ok(()));
    let output = workspace
        .stream_projects::<MockAppRuntime>(&ctx, channel)
        .await
        .unwrap();
    assert_eq!(output.total_returned, 1);

    let id = create_project_output.id;

    // Verify the db entries were created
    let storage = GlobalKvStorage::get(&app_delegate);
    // Check order was stored
    let order_value = storage
        .get(
            &ctx,
            StorageScope::Workspace(workspace.id().inner()),
            &key_project_order(&id),
        )
        .await
        .unwrap()
        .unwrap();
    let order: isize = serde_json::from_value(order_value).unwrap();

    assert_eq!(order, 0);
    // Check expanded_items contains the project id
    let expanded_items_value = storage
        .get(
            &ctx,
            StorageScope::Workspace(workspace.id().inner()),
            KEY_EXPANDED_ITEMS,
        )
        .await
        .unwrap()
        .unwrap();
    let expanded_items: HashSet<ProjectId> = serde_json::from_value(expanded_items_value).unwrap();
    assert!(expanded_items.contains(&id));

    tokio::fs::remove_dir_all(&external_path).await.unwrap();
    cleanup().await;
}
