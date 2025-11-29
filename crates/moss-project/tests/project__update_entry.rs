#![cfg(feature = "integration-tests")]

mod shared;

use moss_applib::mock::MockAppRuntime;
use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use moss_project::{
    dirs,
    models::{
        operations::{CreateResourceInput, UpdateResourceInput},
        primitives::{ResourceClass, ResourceId, ResourceProtocol},
        types::{
            BodyInfo, CreateItemResourceParams, UpdateBodyParams, UpdateDirResourceParams,
            UpdateItemResourceParams,
            http::{
                AddBodyParams, AddFormDataParamParams, AddHeaderParams, AddPathParamParams,
                AddQueryParamParams, AddUrlencodedParamParams, FormDataParamOptions,
                HeaderParamOptions, PathParamOptions, QueryParamOptions, UpdateFormDataParamParams,
                UpdateHeaderParams, UpdatePathParamParams, UpdateQueryParamParams,
                UpdateUrlencodedParamParams, UrlencodedParamOptions,
            },
        },
    },
    storage::{KEY_EXPANDED_ENTRIES, key_resource_order},
};
use moss_storage2::models::primitives::StorageScope;
use moss_testutils::fs_specific::FILENAME_SPECIAL_CHARS;
use moss_text::sanitized::sanitize;
use sapic_runtime::globals::GlobalKvStorage;
use serde_json::{Value as JsonValue, json};
use std::path::{Path, PathBuf};

use crate::shared::{
    RESOURCES_ROOT_DIR, create_test_component_dir_entry, create_test_endpoint_dir_entry,
    create_test_project, random_entry_name,
};

#[tokio::test]
async fn rename_dir_entry_success() {
    let (ctx, app_delegate, project_path, mut project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let old_entry_name = random_entry_name();
    let new_entry_name = random_entry_name();

    let id = create_test_endpoint_dir_entry(&ctx, &mut project, &old_entry_name).await;

    let _ = project
        .update_resource::<MockAppRuntime>(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id,
                path: None,
                name: Some(new_entry_name.clone()),
                order: None,
                expanded: None,
            }),
        )
        .await
        .unwrap();

    // Verify the path has been renamed
    let old_path = resources_dir.join(RESOURCES_ROOT_DIR).join(&old_entry_name);
    let new_path = resources_dir.join(RESOURCES_ROOT_DIR).join(&new_entry_name);
    assert!(!old_path.exists());
    assert!(new_path.exists());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn rename_dir_entry_empty_name() {
    let (ctx, app_delegate, _, mut project, cleanup) = create_test_project().await;

    let old_entry_name = random_entry_name();
    let new_entry_name = "".to_string();

    let id = create_test_component_dir_entry(&ctx, &mut project, &old_entry_name).await;

    let result = project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id,
                path: None,
                name: Some(new_entry_name.clone()),
                order: None,
                expanded: None,
            }),
        )
        .await;

    assert!(result.is_err());

    //Cleanup
    cleanup().await;
}

#[tokio::test]
async fn rename_dir_entry_already_exists() {
    let (ctx, app_delegate, _, mut project, cleanup) = create_test_project().await;
    let first_entry_name = random_entry_name();
    let second_entry_name = random_entry_name();

    let first_id = create_test_component_dir_entry(&ctx, &mut project, &first_entry_name).await;

    let _ = create_test_component_dir_entry(&ctx, &mut project, &second_entry_name).await;

    // Try to rename first entry to the second name
    let result = project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id: first_id,
                path: None,
                name: Some(second_entry_name.clone()),
                order: None,
                expanded: None,
            }),
        )
        .await;

    assert!(result.is_err());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn rename_dir_entry_special_chars_in_name() {
    let (ctx, app_delegate, project_path, mut project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_base_path = PathBuf::from(RESOURCES_ROOT_DIR);

    for special_char in FILENAME_SPECIAL_CHARS {
        let entry_name = random_entry_name();
        let new_entry_name = format!("{}{}", entry_name, special_char);

        let id = create_test_component_dir_entry(&ctx, &mut project, &entry_name).await;

        let result = project
            .update_resource(
                &ctx,
                &app_delegate,
                UpdateResourceInput::Dir(UpdateDirResourceParams {
                    id,
                    path: None,
                    name: Some(new_entry_name.clone()),
                    order: None,
                    expanded: None,
                }),
            )
            .await;

        if result.is_err() {
            // Some special characters might legitimately fail, just skip them
            eprintln!(
                "Skipping special char '{}' due to filesystem limitations",
                special_char
            );
            continue;
        }
        let _ = result.unwrap();

        let expected_dir = resources_dir
            .join(&entry_base_path)
            .join(&sanitize(&new_entry_name));
        dbg!(&expected_dir);
        assert!(expected_dir.exists());
        assert!(expected_dir.is_dir());
    }

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn update_dir_entry_order() {
    let (ctx, app_delegate, _, mut project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&ctx, &mut project, &entry_name).await;

    let _ = project
        .update_resource::<MockAppRuntime>(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: Some(42),
                expanded: None,
            }),
        )
        .await
        .unwrap();

    let storage = GlobalKvStorage::get(&app_delegate);
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
async fn expand_and_collapse_dir_entry() {
    let (ctx, app_delegate, _, mut project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&ctx, &mut project, &entry_name).await;

    let storage = GlobalKvStorage::get(&app_delegate);
    let storage_scope = StorageScope::Project(project.id().inner());

    // Expanding the entry
    let _ = project
        .update_resource::<MockAppRuntime>(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: Some(true),
            }),
        )
        .await
        .unwrap();

    // Check expanded_items contains the entry id
    let expanded_items_value = storage
        .get(storage_scope.clone(), KEY_EXPANDED_ENTRIES)
        .await
        .unwrap()
        .unwrap();
    let expanded_items: Vec<ResourceId> = serde_json::from_value(expanded_items_value).unwrap();
    assert!(expanded_items.contains(&id));

    // Collapsing the entry
    let _ = project
        .update_resource::<MockAppRuntime>(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: Some(false),
            }),
        )
        .await
        .unwrap();

    // Check expanded_items no longer contains the entry id
    let expanded_items_value = storage
        .get(storage_scope.clone(), KEY_EXPANDED_ENTRIES)
        .await
        .unwrap()
        .unwrap();
    let expanded_items: Vec<ResourceId> = serde_json::from_value(expanded_items_value).unwrap();
    assert!(!expanded_items.contains(&id));
    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn move_dir_entry_success() {
    let (ctx, app_delegate, project_path, mut project, cleanup) = create_test_project().await;
    let resources_dir = project_path.join(dirs::RESOURCES_DIR);

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&ctx, &mut project, &entry_name).await;

    // Create a destination_directory named dest
    let _ = create_test_component_dir_entry(&ctx, &mut project, "dest").await;

    let old_dest = PathBuf::from(RESOURCES_ROOT_DIR);
    let new_dest = Path::new(RESOURCES_ROOT_DIR).join("dest");

    // Move entry path from `components/{entry_name}` to `components/dest/{entry_name}`
    let _output = project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id,
                path: Some(new_dest.clone()),
                name: None,
                order: None,
                expanded: None,
            }),
        )
        .await
        .unwrap();

    // Verify the path has been changed
    let old_path = resources_dir.join(old_dest).join(&entry_name);
    let new_path = resources_dir.join(new_dest).join(&entry_name);
    assert!(!old_path.exists());
    assert!(new_path.exists());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn move_dir_entry_nonexistent_destination() {
    let (ctx, app_delegate, _, mut project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();

    let id = create_test_component_dir_entry(&ctx, &mut project, &entry_name).await;

    let new_dest = Path::new(RESOURCES_ROOT_DIR).join("dest");

    // Move entry path from `{entry_name}` to `dest/{entry_name}`
    let result = project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id,
                path: Some(new_dest.clone()),
                name: None,
                order: None,
                expanded: None,
            }),
        )
        .await;

    assert!(result.is_err());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn move_dir_entry_already_exists() {
    let (ctx, app_delegate, _, mut project, cleanup) = create_test_project().await;

    // First create a dest/entry entry
    let dest_name = "dest".to_string();
    let entry_name = "entry".to_string();

    create_test_component_dir_entry(&ctx, &mut project, &dest_name).await;
    let existing_id = create_test_component_dir_entry(&ctx, &mut project, &entry_name).await;

    let dest = Path::new(RESOURCES_ROOT_DIR).join(&dest_name);
    let _ = project
        .update_resource::<MockAppRuntime>(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id: existing_id,
                path: Some(dest.clone()),
                name: None,
                order: None,
                expanded: None,
            }),
        )
        .await
        .unwrap();

    // Create a new entry and try to move it into dest
    let new_id = create_test_component_dir_entry(&ctx, &mut project, &entry_name).await;
    let result = project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Dir(UpdateDirResourceParams {
                id: new_id,
                path: Some(dest.clone()),
                name: None,
                order: None,
                expanded: None,
            }),
        )
        .await;

    assert!(result.is_err());

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn update_item_entry_endpoint_headers() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![AddHeaderParams {
            name: "1".to_string(),
            value: JsonValue::String("1".to_string()),
            order: 1,
            description: Some("1".to_string()),
            options: HeaderParamOptions {
                disabled: false,
                propagate: false,
            },
        }],
        path_params: vec![],
        query_params: vec![],
        body: None,
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let header_id = desc.headers.first().unwrap().id.clone();

    // Test update header
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![UpdateHeaderParams {
                    id: header_id.clone(),
                    name: Some("2".to_string()),
                    value: Some(ChangeJsonValue::Update(JsonValue::String("2".to_string()))),
                    order: Some(2),
                    description: Some(ChangeString::Update("2".to_string())),
                    options: Some(HeaderParamOptions {
                        disabled: true,
                        propagate: true,
                    }),
                }],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: None,
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let header = desc.headers.first().unwrap();

    assert_eq!(header.name, "2");
    assert_eq!(header.value, JsonValue::String("2".to_string()));
    assert_eq!(header.order, Some(2));
    assert_eq!(header.description, Some("2".to_string()));
    assert_eq!(header.disabled, true);
    assert_eq!(header.propagate, true);

    // Test delete header

    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![header_id.clone()],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: None,
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    assert!(desc.headers.is_empty());

    // Test add header
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![AddHeaderParams {
                    name: "3".to_string(),
                    value: JsonValue::String("3".to_string()),
                    order: 3,
                    description: Some("3".to_string()),
                    options: HeaderParamOptions {
                        disabled: false,
                        propagate: false,
                    },
                }],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: None,
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    assert_eq!(desc.headers.len(), 1);
    let header = desc.headers.first().unwrap();
    assert_eq!(header.name, "3");
    assert_eq!(header.value, JsonValue::String("3".to_string()));
    assert_eq!(header.order, Some(3));
    assert_eq!(header.description, Some("3".to_string()));
    assert_eq!(header.disabled, false);
    assert_eq!(header.propagate, false);

    cleanup().await;
}

#[tokio::test]
async fn update_item_entry_endpoint_path_params() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![AddPathParamParams {
            name: "1".to_string(),
            value: JsonValue::String("1".to_string()),
            order: 1,
            description: Some("1".to_string()),
            options: PathParamOptions {
                disabled: false,
                propagate: false,
            },
        }],
        query_params: vec![],
        body: None,
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let path_param_id = desc.path_params.first().unwrap().id.clone();

    // Test update header
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![UpdatePathParamParams {
                    id: path_param_id.clone(),
                    name: Some("2".to_string()),
                    value: Some(ChangeJsonValue::Update(JsonValue::String("2".to_string()))),
                    order: Some(2),
                    description: Some(ChangeString::Update("2".to_string())),
                    options: Some(PathParamOptions {
                        disabled: true,
                        propagate: true,
                    }),
                }],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: None,
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let path_param = desc.path_params.first().unwrap();

    assert_eq!(path_param.name, "2");
    assert_eq!(path_param.value, JsonValue::String("2".to_string()));
    assert_eq!(path_param.order, Some(2));
    assert_eq!(path_param.description, Some("2".to_string()));
    assert_eq!(path_param.disabled, true);
    assert_eq!(path_param.propagate, true);

    // Test delete header
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![path_param_id.clone()],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: None,
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    assert!(desc.path_params.is_empty());

    // Test add header
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![AddPathParamParams {
                    name: "3".to_string(),
                    value: JsonValue::String("3".to_string()),
                    order: 3,
                    description: Some("3".to_string()),
                    options: PathParamOptions {
                        disabled: false,
                        propagate: false,
                    },
                }],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: None,
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    assert_eq!(desc.path_params.len(), 1);
    let path_param = desc.path_params.first().unwrap();
    assert_eq!(path_param.name, "3");
    assert_eq!(path_param.value, JsonValue::String("3".to_string()));
    assert_eq!(path_param.order, Some(3));
    assert_eq!(path_param.description, Some("3".to_string()));
    assert_eq!(path_param.disabled, false);
    assert_eq!(path_param.propagate, false);

    cleanup().await;
}

#[tokio::test]
async fn update_item_entry_endpoint_query_params() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![AddQueryParamParams {
            name: "1".to_string(),
            value: JsonValue::String("1".to_string()),
            order: 1,
            description: Some("1".to_string()),
            options: QueryParamOptions {
                disabled: false,
                propagate: false,
            },
        }],
        body: None,
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let query_param_id = desc.query_params.first().unwrap().id.clone();

    // Test update header
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![UpdateQueryParamParams {
                    id: query_param_id.clone(),
                    name: Some("2".to_string()),
                    value: Some(ChangeJsonValue::Update(JsonValue::String("2".to_string()))),
                    order: Some(2),
                    description: Some(ChangeString::Update("2".to_string())),
                    options: Some(QueryParamOptions {
                        disabled: true,
                        propagate: true,
                    }),
                }],
                query_params_to_remove: vec![],
                body: None,
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let query_param = desc.query_params.first().unwrap();

    assert_eq!(query_param.name, "2");
    assert_eq!(query_param.value, JsonValue::String("2".to_string()));
    assert_eq!(query_param.order, Some(2));
    assert_eq!(query_param.description, Some("2".to_string()));
    assert_eq!(query_param.disabled, true);
    assert_eq!(query_param.propagate, true);

    // Test delete header
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![query_param_id.clone()],
                body: None,
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    assert!(desc.query_params.is_empty());

    // Test add header
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![AddQueryParamParams {
                    name: "3".to_string(),
                    value: JsonValue::String("3".to_string()),
                    order: 3,
                    description: Some("3".to_string()),
                    options: QueryParamOptions {
                        disabled: false,
                        propagate: false,
                    },
                }],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: None,
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    assert_eq!(desc.query_params.len(), 1);
    let query_param = desc.query_params.first().unwrap();
    assert_eq!(query_param.name, "3");
    assert_eq!(query_param.value, JsonValue::String("3".to_string()));
    assert_eq!(query_param.order, Some(3));
    assert_eq!(query_param.description, Some("3".to_string()));
    assert_eq!(query_param.disabled, false);
    assert_eq!(query_param.propagate, false);

    cleanup().await;
}

#[tokio::test]
async fn test_item_entry_endpoint_remove_body() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::FormData(vec![AddFormDataParamParams {
            name: "1".to_string(),
            value: JsonValue::String("1".to_string()),
            order: 1,
            description: None,
            options: FormDataParamOptions {
                disabled: false,
                propagate: false,
            },
            id: None,
        }])),
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    // Test remove body
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::Remove),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    assert!(desc.body.is_none());

    cleanup().await;
}

#[tokio::test]
async fn test_item_entry_endpoint_update_text() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Text("Before".to_string())),
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    // Test update body text
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::Text("After".to_string())),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    // An extra \n is added during deserialization
    assert_eq!(desc.body, Some(BodyInfo::Text("After\n".to_string())));

    cleanup().await;
}

#[tokio::test]
async fn test_item_entry_endpoint_update_json() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Json(json!( {"before": "true"} ))),
    });

    let new_json = json!( {"after": "true"} );
    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    // Test update body json
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::Json(new_json.clone())),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    assert_eq!(desc.body, Some(BodyInfo::Json(new_json.clone())));

    cleanup().await;
}

#[tokio::test]
async fn test_item_entry_endpoint_update_xml() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Xml("<before></before>".to_string())),
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;
    // Test update body xml
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::Xml("<after></after>".to_string())),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    // An extra \n is added during deserialization
    assert_eq!(
        desc.body,
        Some(BodyInfo::Xml("<after></after>\n".to_string()))
    );

    cleanup().await;
}

#[tokio::test]
async fn test_item_entry_endpoint_update_binary() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;

    let entry_name = random_entry_name();
    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Binary(PathBuf::from("/before"))),
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    // Test update body binary
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::Binary(PathBuf::from("/after"))),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    assert_eq!(desc.body, Some(BodyInfo::Binary(PathBuf::from("/after"))));

    cleanup().await;
}

#[tokio::test]
async fn test_item_entry_endpoint_update_urlencoded() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;
    let entry_name = random_entry_name();

    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::Urlencoded(vec![])),
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    let before = AddUrlencodedParamParams {
        name: "before".to_string(),
        value: JsonValue::String("before".to_string()),
        order: 1,
        description: Some("before".to_string()),
        options: UrlencodedParamOptions {
            disabled: false,
            propagate: false,
        },
        id: None,
    };

    // Test add urlencoded param
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::Urlencoded {
                    params_to_add: vec![before.clone()],
                    params_to_update: vec![],
                    params_to_remove: vec![],
                }),
            }),
        )
        .await
        .unwrap();
    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let urlencoded = if let Some(BodyInfo::Urlencoded(urlencoded)) = desc.body {
        urlencoded
    } else {
        panic!("Incorrect body type");
    };
    assert_eq!(urlencoded.len(), 1);

    let param = urlencoded.get(0).unwrap();
    assert_eq!(param.name, before.name);
    assert_eq!(param.value, before.value);
    assert_eq!(param.order, Some(before.order));
    assert_eq!(param.description, before.description);
    assert_eq!(param.disabled, before.options.disabled);
    assert_eq!(param.propagate, before.options.propagate);

    // Test update urlencoded param
    let param_id = param.id.clone();
    let after = UpdateUrlencodedParamParams {
        id: param_id.clone(),
        name: Some("after".to_string()),
        value: Some(ChangeJsonValue::Update(JsonValue::String(
            "after".to_string(),
        ))),
        order: Some(1),
        description: Some(ChangeString::Remove),
        options: Some(UrlencodedParamOptions {
            disabled: true,
            propagate: true,
        }),
    };

    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::Urlencoded {
                    params_to_add: vec![],
                    params_to_update: vec![after.clone()],
                    params_to_remove: vec![],
                }),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let urlencoded = if let Some(BodyInfo::Urlencoded(urlencoded)) = desc.body {
        urlencoded
    } else {
        panic!("Incorrect body type");
    };
    assert_eq!(urlencoded.len(), 1);

    let param = urlencoded.get(0).unwrap();
    assert_eq!(param.name, after.name.unwrap());
    assert_eq!(param.value, JsonValue::String("after".to_string()));
    assert_eq!(param.order, after.order);
    assert_eq!(param.description, None);
    assert_eq!(param.disabled, true);
    assert_eq!(param.propagate, true);

    // Test remove urlencoded param
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::Urlencoded {
                    params_to_add: vec![],
                    params_to_update: vec![],
                    params_to_remove: vec![param_id.clone()],
                }),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let urlencoded = if let Some(BodyInfo::Urlencoded(urlencoded)) = desc.body {
        urlencoded
    } else {
        panic!("Incorrect body type");
    };
    assert_eq!(urlencoded.len(), 0);

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn test_item_entry_endpoint_update_formdata() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;
    let entry_name = random_entry_name();

    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::FormData(vec![])),
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    let before = AddFormDataParamParams {
        name: "before".to_string(),
        value: JsonValue::String("before".to_string()),
        order: 1,
        description: Some("before".to_string()),
        options: FormDataParamOptions {
            disabled: false,
            propagate: false,
        },
        id: None,
    };

    // Test add formdata param
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::FormData {
                    params_to_add: vec![before.clone()],
                    params_to_update: vec![],
                    params_to_remove: vec![],
                }),
            }),
        )
        .await
        .unwrap();
    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let formdata = if let Some(BodyInfo::FormData(formdata)) = desc.body {
        formdata
    } else {
        panic!("Incorrect body type");
    };
    assert_eq!(formdata.len(), 1);

    let param = formdata.get(0).unwrap();
    assert_eq!(param.name, before.name);
    assert_eq!(param.value, before.value);
    assert_eq!(param.order, Some(before.order));
    assert_eq!(param.description, before.description);
    assert_eq!(param.disabled, before.options.disabled);
    assert_eq!(param.propagate, before.options.propagate);

    // Test update formdata param
    let param_id = param.id.clone();
    let after = UpdateFormDataParamParams {
        id: param_id.clone(),
        name: Some("after".to_string()),
        value: Some(ChangeJsonValue::Update(JsonValue::String(
            "after".to_string(),
        ))),
        order: Some(1),
        description: Some(ChangeString::Remove),
        options: Some(FormDataParamOptions {
            disabled: true,
            propagate: true,
        }),
    };

    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::FormData {
                    params_to_add: vec![],
                    params_to_update: vec![after.clone()],
                    params_to_remove: vec![],
                }),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let formdata = if let Some(BodyInfo::FormData(formdata)) = desc.body {
        formdata
    } else {
        panic!("Incorrect body type");
    };
    assert_eq!(formdata.len(), 1);

    let param = formdata.get(0).unwrap();
    assert_eq!(param.name, after.name.unwrap());
    assert_eq!(param.value, JsonValue::String("after".to_string()));
    assert_eq!(param.order, after.order);
    assert_eq!(param.description, None);
    assert_eq!(param.disabled, true);
    assert_eq!(param.propagate, true);

    // Test remove formdata param
    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::FormData {
                    params_to_add: vec![],
                    params_to_update: vec![],
                    params_to_remove: vec![param_id.clone()],
                }),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let formdata = if let Some(BodyInfo::FormData(formdata)) = desc.body {
        formdata
    } else {
        panic!("Incorrect body type");
    };
    assert_eq!(formdata.len(), 0);

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn test_item_entry_endpoint_update_change_body_type() {
    let (ctx, app_delegate, _, project, cleanup) = create_test_project().await;
    let entry_name = random_entry_name();

    let input = CreateResourceInput::Item(CreateItemResourceParams {
        path: Default::default(),
        class: ResourceClass::Endpoint,
        name: entry_name.clone(),
        order: 0,
        protocol: Some(ResourceProtocol::Get),
        headers: vec![],
        path_params: vec![],
        query_params: vec![],
        body: Some(AddBodyParams::FormData(vec![])),
    });

    let id = project
        .create_resource::<MockAppRuntime>(&ctx, input)
        .await
        .unwrap()
        .id;

    project
        .update_resource(
            &ctx,
            &app_delegate,
            UpdateResourceInput::Item(UpdateItemResourceParams {
                id: id.clone(),
                path: None,
                name: None,
                order: None,
                expanded: None,
                protocol: None,
                headers_to_add: vec![],
                headers_to_update: vec![],
                headers_to_remove: vec![],
                path_params_to_add: vec![],
                path_params_to_update: vec![],
                path_params_to_remove: vec![],
                query_params_to_add: vec![],
                query_params_to_update: vec![],
                query_params_to_remove: vec![],
                body: Some(UpdateBodyParams::Urlencoded {
                    params_to_add: vec![],
                    params_to_update: vec![],
                    params_to_remove: vec![],
                }),
            }),
        )
        .await
        .unwrap();

    let desc = project
        .describe_resource(&ctx, &app_delegate, id.clone())
        .await
        .unwrap();
    let urlencoded = if let Some(BodyInfo::Urlencoded(urlencoded)) = desc.body {
        urlencoded
    } else {
        panic!("Incorrect body type");
    };
    assert_eq!(urlencoded.len(), 0);

    cleanup().await;
}
