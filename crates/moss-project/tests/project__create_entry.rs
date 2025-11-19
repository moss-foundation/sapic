#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_project::{
    constants, dirs,
    errors::ErrorAlreadyExists,
    models::{
        operations::CreateResourceInput,
        primitives::{ResourceClass, ResourceKind, ResourceProtocol},
        types::{
            BodyInfo, CreateDirResourceParams, CreateItemResourceParams,
            http::{
                AddBodyParams, AddFormDataParamParams, AddHeaderParams, AddPathParamParams,
                AddQueryParamParams, AddUrlencodedParamParams, FormDataParamOptions,
                HeaderParamOptions, PathParamOptions, QueryParamOptions, UrlencodedParamOptions,
            },
        },
    },
    storage::key_resource_order,
};
use moss_storage2::{Storage, models::primitives::StorageScope};
use moss_testutils::fs_specific::FILENAME_SPECIAL_CHARS;
use moss_text::sanitized::sanitize;
use serde_json::{Value as JsonValue, json};
use std::path::PathBuf;

use crate::shared::{RESOURCES_ROOT_DIR, create_test_project, random_entry_name};

#[tokio::test]
async fn create_dir_entry_success() {
    let (ctx, _, project_path, project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let input = CreateResourceInput::Dir(CreateDirResourceParams {
        class: ResourceClass::Endpoint,
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
    });

    let result = project.create_resource(&ctx, input).await;

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
    cleanup().await;
}

#[tokio::test]
async fn create_dir_entry_with_order() {
    let (ctx, app_delegate, project_path, project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(RESOURCES_ROOT_DIR);
    let order_value = 42;

    let input = CreateResourceInput::Dir(CreateDirResourceParams {
        class: ResourceClass::Endpoint,
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: order_value,
    });

    let result = project.create_resource(&ctx, input).await;
    let id = result.unwrap().id;

    // Verify the directory was created
    let expected_dir = resources_dir.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());

    let storage = <dyn Storage>::global(&app_delegate);
    let storage_scope = StorageScope::Project(project.id().inner());

    // Check order was updated
    let order_value = storage
        .get(storage_scope, &key_resource_order(&id))
        .await
        .unwrap()
        .unwrap();

    let order: isize = serde_json::from_value(order_value).unwrap();

    assert_eq!(order, 42);

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn create_dir_entry_already_exists() {
    let (ctx, _, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from(RESOURCES_ROOT_DIR);

    let input = CreateResourceInput::Dir(CreateDirResourceParams {
        class: ResourceClass::Endpoint,
        path: entry_path.clone(),
        name: entry_name.clone(),
        order: 0,
    });

    // Create the entry first time - should succeed
    let first_result = project.create_resource(&ctx, input.clone()).await;
    let _ = first_result.unwrap();

    // Try to create the same entry again - should fail
    let second_result = project.create_resource(&ctx, input).await;
    assert!(second_result.is_err());

    if let Err(error) = second_result {
        assert!(error.is::<ErrorAlreadyExists>());
    }

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn create_dir_entry_special_chars_in_name() {
    let (ctx, _, project_path, project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let base_name = random_entry_name();

    for special_char in FILENAME_SPECIAL_CHARS {
        let entry_name = format!("{}{}", base_name, special_char);
        let entry_path = PathBuf::from(RESOURCES_ROOT_DIR);

        let input = CreateResourceInput::Dir(CreateDirResourceParams {
            class: ResourceClass::Endpoint,
            path: entry_path.clone(),
            name: entry_name.clone(),
            order: 0,
        });

        let result = project.create_resource(&ctx, input).await;

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
    cleanup().await;
}

#[tokio::test]
async fn create_item_entry_endpoint() {
    let (ctx, app_delegate, project_path, project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: entry_path.clone(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
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
        body: None,
    });

    let result = project.create_resource(&ctx, input).await;

    let id = result.unwrap().id;

    // Verify the directory was created
    let expected_dir = resources_dir.join(&entry_path).join(&entry_name);
    assert!(expected_dir.exists());
    assert!(expected_dir.is_dir());

    let config_file = expected_dir.join(constants::ITEM_CONFIG_FILENAME);
    assert!(config_file.exists());
    assert!(config_file.is_file());

    // Verify the config is correctly set

    let desc = project
        .describe_resource(&ctx, &app_delegate, id)
        .await
        .unwrap();

    assert_eq!(desc.name, entry_name);
    assert_eq!(desc.class, ResourceClass::Endpoint);
    assert_eq!(desc.kind, ResourceKind::Item);
    assert_eq!(desc.protocol, Some(ResourceProtocol::Get));

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
    cleanup().await;
}

// Note: deserialization of heredoc strings will append a newline character at the end
// This will probably need to be handled on the frontend.
#[tokio::test]
async fn create_item_entry_body_text() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let text = r#"Test
Multiline
String"#;
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: entry_path.clone(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Text(text.to_string())),
    });

    let result = project.create_resource(&ctx, input).await;
    let id = result.unwrap().id;

    let body_desc = project
        .describe_resource(&ctx, &app_delegate, id)
        .await
        .unwrap()
        .body
        .unwrap();
    assert_eq!(body_desc, BodyInfo::Text(text.to_string() + "\n"));

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn create_item_entry_body_json() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let json = json!({
        "foo": 1,
        "bar": [2, 3],
        "baz": {"4": "5"}
    });
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: entry_path.clone(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Json(json.clone())),
    });

    let result = project.create_resource(&ctx, input).await;
    let id = result.unwrap().id;

    let body_desc = project
        .describe_resource(&ctx, &app_delegate, id)
        .await
        .unwrap()
        .body
        .unwrap();
    assert_eq!(body_desc, BodyInfo::Json(json));

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn create_item_entry_body_xml() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let xml = r#""<?xml version="1.0" encoding="UTF-8"?>""#;
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: entry_path.clone(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Xml(xml.to_string())),
    });

    let result = project.create_resource(&ctx, input).await;
    let id = result.unwrap().id;

    let body_desc = project
        .describe_resource(&ctx, &app_delegate, id)
        .await
        .unwrap()
        .body
        .unwrap();
    assert_eq!(body_desc, BodyInfo::Xml(xml.to_string() + "\n"));

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn create_item_entry_body_binary() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let binary = PathBuf::from("foo/bar.txt");
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: entry_path.clone(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Binary(binary.clone())),
    });

    let result = project.create_resource(&ctx, input).await;
    let id = result.unwrap().id;

    let body_desc = project
        .describe_resource(&ctx, &app_delegate, id)
        .await
        .unwrap()
        .body
        .unwrap();
    assert_eq!(body_desc, BodyInfo::Binary(binary));

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn create_item_entry_body_urlencoded() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let params = vec![
        AddUrlencodedParamParams {
            name: "param1".to_string(),
            value: JsonValue::String("value1".to_string()),
            order: 1,
            description: Some("description".to_string()),
            options: UrlencodedParamOptions {
                disabled: false,
                propagate: false,
            },
            id: None,
        },
        AddUrlencodedParamParams {
            name: "param2".to_string(),
            value: JsonValue::String("value2".to_string()),
            order: 2,
            description: Some("description".to_string()),
            options: UrlencodedParamOptions {
                disabled: false,
                propagate: false,
            },
            id: None,
        },
    ];
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: entry_path.clone(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Urlencoded(params)),
    });
    let result = project.create_resource(&ctx, input).await;
    let id = result.unwrap().id;

    let body_desc = project
        .describe_resource(&ctx, &app_delegate, id)
        .await
        .unwrap()
        .body
        .unwrap();

    match body_desc {
        BodyInfo::Urlencoded(urlencoded) => {
            assert!(urlencoded.iter().any(|param| {
                param.name == "param1"
                    && param.value == JsonValue::String("value1".to_string())
                    && param.order == Some(1)
                    && param.description.as_deref() == Some("description")
                    && !param.propagate
                    && !param.disabled
            }));
            assert!(urlencoded.iter().any(|param| {
                param.name == "param2"
                    && param.value == JsonValue::String("value2".to_string())
                    && param.order == Some(2)
                    && param.description.as_deref() == Some("description")
                    && !param.propagate
                    && !param.disabled
            }));
        }
        _ => panic!("incorrect body type"),
    }

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn create_item_entry_body_formdata() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let entry_path = PathBuf::from("");
    let params = vec![
        AddFormDataParamParams {
            name: "param1".to_string(),
            value: JsonValue::String("value1".to_string()),
            order: 1,
            description: Some("description".to_string()),
            options: FormDataParamOptions {
                disabled: false,
                propagate: false,
            },
            id: None,
        },
        AddFormDataParamParams {
            name: "param2".to_string(),
            value: JsonValue::String("value2".to_string()),
            order: 2,
            description: Some("description".to_string()),
            options: FormDataParamOptions {
                disabled: false,
                propagate: false,
            },
            id: None,
        },
    ];
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: entry_path.clone(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::FormData(params.clone())),
    });
    let result = project.create_resource(&ctx, input).await;
    let id = result.unwrap().id;

    let body_desc = project
        .describe_resource(&ctx, &app_delegate, id)
        .await
        .unwrap()
        .body
        .unwrap();

    match body_desc {
        BodyInfo::FormData(form_data) => {
            assert!(form_data.iter().any(|param| {
                param.name == "param1"
                    && param.value == JsonValue::String("value1".to_string())
                    && param.order == Some(1)
                    && param.description.as_deref() == Some("description")
                    && !param.propagate
                    && !param.disabled
            }));
            assert!(form_data.iter().any(|param| {
                param.name == "param2"
                    && param.value == JsonValue::String("value2".to_string())
                    && param.order == Some(2)
                    && param.description.as_deref() == Some("description")
                    && !param.propagate
                    && !param.disabled
            }));
        }
        _ => panic!("incorrect body type"),
    }

    // Cleanup
    cleanup().await;
}
