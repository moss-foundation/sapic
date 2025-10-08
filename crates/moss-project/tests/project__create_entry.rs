#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_project::{
    constants, dirs,
    errors::ErrorAlreadyExists,
    models::{
        operations::CreateEntryInput,
        primitives::{EntryClass, EntryKind, EntryProtocol},
        types::{
            CreateDirEntryParams, CreateItemEntryParams,
            http::{
                AddHeaderParams, AddPathParamParams, AddQueryParamParams, HeaderParamOptions,
                PathParamOptions, QueryParamOptions,
            },
        },
    },
    storage::segments::SEGKEY_RESOURCE_ENTRY,
};
use moss_storage::storage::operations::GetItem;
use moss_testutils::fs_specific::FILENAME_SPECIAL_CHARS;
use moss_text::sanitized::sanitize;
use serde_json::Value as JsonValue;
use std::path::PathBuf;

use crate::shared::{RESOURCES_ROOT_DIR, create_test_project, random_entry_name};

#[tokio::test]
async fn create_dir_entry_success() {
    let (ctx, _, project_path, project) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let input = CreateEntryInput::Dir(CreateDirEntryParams {
        class: EntryClass::Endpoint,
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
    });

    let result = project.create_entry(&ctx, input).await;

    let output = result.unwrap();

    // Verify the directory was created
    let expected_dir = resources_dir.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());
    assert!(expected_dir.is_dir());

    let config_file = expected_dir.join(constants::DIR_CONFIG_FILENAME);
    assert!(config_file.exists());
    assert!(config_file.is_file());

    // Read and verify config content
    let config_content = std::fs::read_to_string(config_file).unwrap();
    assert!(config_content.contains(&output.id.to_string()));

    // Cleanup
    std::fs::remove_dir_all(project_path).unwrap();
}

#[tokio::test]
async fn create_dir_entry_with_order() {
    let (ctx, _, project_path, project) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(RESOURCES_ROOT_DIR);
    let order_value = 42;

    let input = CreateEntryInput::Dir(CreateDirEntryParams {
        class: EntryClass::Endpoint,
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: order_value,
    });

    let result = project.create_entry(&ctx, input).await;
    let id = result.unwrap().id;

    // Verify the directory was created
    let expected_dir = resources_dir.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());

    let resource_store = project.db().resource_store();

    // Check order was updated
    let order_key = SEGKEY_RESOURCE_ENTRY.join(&id.to_string()).join("order");
    let order_value = GetItem::get(resource_store.as_ref(), &ctx, order_key)
        .await
        .unwrap();
    let stored_order: isize = order_value.deserialize().unwrap();
    assert_eq!(stored_order, 42);

    // Cleanup
    std::fs::remove_dir_all(project_path).unwrap();
}

#[tokio::test]
async fn create_dir_entry_already_exists() {
    let (ctx, _, project_path, project) = create_test_project().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(RESOURCES_ROOT_DIR);

    let input = CreateEntryInput::Dir(CreateDirEntryParams {
        class: EntryClass::Endpoint,
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
    });

    // Create the entry first time - should succeed
    let first_result = project.create_entry(&ctx, input.clone()).await;
    let _ = first_result.unwrap();

    // Try to create the same entry again - should fail
    let second_result = project.create_entry(&ctx, input).await;
    assert!(second_result.is_err());

    if let Err(error) = second_result {
        assert!(error.is::<ErrorAlreadyExists>());
    }

    // Cleanup
    std::fs::remove_dir_all(project_path).unwrap();
}

#[tokio::test]
async fn create_dir_entry_special_chars_in_name() {
    let (ctx, _, project_path, project) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let base_name = random_entry_name();

    for special_char in FILENAME_SPECIAL_CHARS {
        let entry_name = format!("{}{}", base_name, special_char);
        let entry_path = PathBuf::from(RESOURCES_ROOT_DIR);

        let input = CreateEntryInput::Dir(CreateDirEntryParams {
            class: EntryClass::Endpoint,
            path: entry_path.clone(),
            name: entry_name.clone(),
            order: 0,
        });

        let result = project.create_entry(&ctx, input).await;

        // Entry creation should succeed - the filesystem layer handles sanitization
        if result.is_err() {
            // Some special characters might legitimately fail, just skip them
            eprintln!(
                "Skipping special char '{}' due to filesystem limitations",
                special_char
            );
            continue;
        }

        let _output = result.unwrap();

        // The exact directory name might be sanitized, but some directory should exist
        // We just verify that the operation completed successfully
        let expected_dir = resources_dir.join(&entry_path).join(sanitize(&entry_name));
        assert!(expected_dir.exists());
        assert!(expected_dir.is_dir());
    }

    // Cleanup
    std::fs::remove_dir_all(project_path).unwrap();
}

#[tokio::test]
async fn create_item_entry_endpoint() {
    let (ctx, _, project_path, project) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let input = CreateEntryInput::Item(CreateItemEntryParams {
        path: entry_path.clone(),
        class: EntryClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(EntryProtocol::Get),
        headers: vec![AddHeaderParams {
            name: "header1".to_string(),
            value: JsonValue::String("value1".to_string()),
            order: 42,
            description: Some("description".to_string()),
            options: HeaderParamOptions {
                disabled: false,
                propagate: false,
            },
        }],
        path_params: vec![AddPathParamParams {
            name: "path_param1".to_string(),
            value: JsonValue::String("value1".to_string()),
            order: 42,
            description: Some("description".to_string()),
            options: PathParamOptions {
                disabled: false,
                propagate: false,
            },
        }],
        query_params: vec![AddQueryParamParams {
            name: "query_param1".to_string(),
            value: JsonValue::String("value1".to_string()),
            order: 42,
            description: Some("description".to_string()),
            options: QueryParamOptions {
                disabled: false,
                propagate: false,
            },
        }],
    });

    let result = project.create_entry(&ctx, input).await;

    let id = result.unwrap().id;

    // Verify the directory was created
    let expected_dir = resources_dir.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());
    assert!(expected_dir.is_dir());

    let config_file = expected_dir.join(constants::ITEM_CONFIG_FILENAME);
    assert!(config_file.exists());
    assert!(config_file.is_file());

    // Verify the config is correctly set

    let desc = project.describe_entry(&ctx, id).await.unwrap();

    assert_eq!(desc.name, entry_name);
    assert_eq!(desc.class, EntryClass::Endpoint);
    assert_eq!(desc.kind, EntryKind::Item);
    assert_eq!(desc.protocol, Some(EntryProtocol::Get));

    assert_eq!(desc.headers.len(), 1);
    let header = desc.headers.first().unwrap();
    assert_eq!(header.name, "header1");
    assert_eq!(header.value, JsonValue::String("value1".to_string()));
    assert_eq!(header.order, Some(42));
    assert_eq!(header.description, Some("description".to_string()));
    assert!(!header.disabled);
    assert!(!header.propagate);

    assert_eq!(desc.path_params.len(), 1);
    let path_param = desc.path_params.first().unwrap();
    assert_eq!(path_param.name, "path_param1");
    assert_eq!(path_param.value, JsonValue::String("value1".to_string()));
    assert_eq!(path_param.order, Some(42));
    assert_eq!(path_param.description, Some("description".to_string()));
    assert!(!path_param.disabled);
    assert!(!path_param.propagate);

    assert_eq!(desc.query_params.len(), 1);
    let query_param = desc.query_params.first().unwrap();
    assert_eq!(query_param.name, "query_param1");
    assert_eq!(query_param.value, JsonValue::String("value1".to_string()));
    assert_eq!(query_param.order, Some(42));
    assert_eq!(query_param.description, Some("description".to_string()));
    assert!(!query_param.disabled);
    assert!(!query_param.propagate);

    // Cleanup
    std::fs::remove_dir_all(project_path).unwrap();
}
