#![cfg(feature = "integration-tests")]

use moss_applib::mock::MockAppRuntime;
use moss_project::{
    constants, dirs,
    models::{
        operations::{BatchCreateResourceInput, BatchCreateResourceKind},
        primitives::ResourceClass,
        types::{CreateDirResourceParams, CreateItemResourceParams},
    },
};
use std::path::PathBuf;

use crate::shared::{RESOURCES_ROOT_DIR, create_test_project, random_entry_name};

pub mod shared;

#[tokio::test]
async fn batch_create_entry_success() {
    let (ctx, _, project_path, project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_base_path = PathBuf::from(RESOURCES_ROOT_DIR);

    // components/{outer_name}
    // components/{outer_name}/{inner_name}

    let outer_name = random_entry_name();
    let inner_name = random_entry_name();
    let outer_input = BatchCreateResourceKind::Dir(CreateDirResourceParams {
        class: ResourceClass::Endpoint,
        path: entry_base_path.clone(),
        name: outer_name.clone(),
        order: 0,
    });
    let inner_input = BatchCreateResourceKind::Item(CreateItemResourceParams {
        class: ResourceClass::Endpoint,
        path: entry_base_path.join(&outer_name),
        name: inner_name.clone(),
        order: 0,
        protocol: None,
        query_params: vec![],
        path_params: vec![],
        headers: vec![],
        body: None,
        url: None,
    });
    let input = BatchCreateResourceInput {
        // Make sure that the order is correctly sorted
        resources: vec![inner_input, outer_input],
    };

    let output = project
        .batch_create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap();
    assert_eq!(output.ids.len(), 2);

    // Verify the directories were created
    let outer_dir = resources_dir.join(&entry_base_path).join(&outer_name);
    assert!(outer_dir.exists());
    assert!(outer_dir.is_dir());
    let outer_config = outer_dir.join(constants::DIR_CONFIG_FILENAME);
    assert!(outer_config.exists());
    assert!(outer_config.is_file());

    let inner_dir = outer_dir.join(&inner_name);
    assert!(inner_dir.exists());
    assert!(inner_dir.is_dir());
    let inner_config = inner_dir.join(constants::ITEM_CONFIG_FILENAME);
    assert!(inner_config.exists());
    assert!(inner_config.is_file());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn batch_create_entry_missing_parent() {
    let (ctx, _, _, project, cleanup) = create_test_project().await;

    let entry_base_path = PathBuf::from(RESOURCES_ROOT_DIR);
    let inner_name = random_entry_name();

    // Try creating components/parent/{inner_name}
    let inner_input = BatchCreateResourceKind::Item(CreateItemResourceParams {
        class: ResourceClass::Endpoint,
        path: entry_base_path.join("parent"),
        name: inner_name.clone(),
        order: 0,
        protocol: None,
        query_params: vec![],
        path_params: vec![],
        headers: vec![],
        body: None,
        url: None,
    });
    let input = BatchCreateResourceInput {
        resources: vec![inner_input],
    };

    let result = project
        .batch_create_resource::<MockAppRuntime>(&ctx, input)
        .await;
    assert!(result.is_err());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn batch_create_entry_empty_input() {
    let (ctx, _, _, project, cleanup) = create_test_project().await;

    let input = BatchCreateResourceInput { resources: vec![] };
    let output = project
        .batch_create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap();

    assert_eq!(output.ids.len(), 0);

    // Cleanup
    cleanup().await;
}
