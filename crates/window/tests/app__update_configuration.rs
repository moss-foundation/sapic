// TODO: restore this in the crate where these operations will be moved.

// #![cfg(feature = "integration-tests")]

// pub mod shared;

// use moss_configuration::models::primitives::ConfigurationTarget;
// use serde_json::{Value as JsonValue, json};
// use tauri::Listener;
// use window::{
//     constants::ON_DID_CHANGE_CONFIGURATION_CHANNEL,
//     models::{
//         events::OnDidChangeConfigurationForFrontend, operations::UpdateConfigurationInput,
//         types::UpdateConfigurationParams,
//     },
// };

// use crate::shared::set_up_test_app;
// use std::{
//     sync::{
//         Arc, Mutex,
//         atomic::{AtomicBool, Ordering},
//     },
//     time::Duration,
// };

// #[tokio::test]
// async fn update_configuration_profile_string_value_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Set up event listener to capture the emitted event
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Update configuration with string value
//     let key = "testKey";
//     let value = JsonValue::String("testValue".to_string());
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.to_string(),
//                     value: value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(std::time::Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(received_event_data.affected_keys, vec![key.to_string()]);
//     assert_eq!(received_event_data.changes.get(key), Some(&value));

//     // Verify configuration was updated
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     assert!(configuration.keys.contains(&key.to_string()));
//     assert_eq!(configuration.contents.get(key), Some(&value));

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_profile_number_value_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Set up event listener to capture the emitted event
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Update configuration with number value
//     let key = "numericKey";
//     let value = JsonValue::Number(serde_json::Number::from(42));
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.to_string(),
//                     value: value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(received_event_data.affected_keys, vec![key.to_string()]);
//     assert_eq!(received_event_data.changes.get(key), Some(&value));

//     // Verify configuration was updated
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     assert!(configuration.keys.contains(&key.to_string()));
//     assert_eq!(configuration.contents.get(key), Some(&value));

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_profile_boolean_value_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Set up event listener to capture the emitted event
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Update configuration with boolean value
//     let key = "booleanKey";
//     let value = JsonValue::Bool(true);
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.to_string(),
//                     value: value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(received_event_data.affected_keys, vec![key.to_string()]);
//     assert_eq!(received_event_data.changes.get(key), Some(&value));

//     // Verify configuration was updated
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     assert!(configuration.keys.contains(&key.to_string()));
//     assert_eq!(configuration.contents.get(key), Some(&value));

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_profile_object_value_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Set up event listener to capture the emitted event
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Update configuration with object value
//     let key = "objectKey";
//     let value = json!({
//         "nested": {
//             "property": "value",
//             "number": 123,
//             "enabled": false
//         }
//     });
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.to_string(),
//                     value: value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(received_event_data.affected_keys, vec![key.to_string()]);
//     assert_eq!(received_event_data.changes.get(key), Some(&value));

//     // Verify configuration was updated
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     assert!(configuration.keys.contains(&key.to_string()));
//     assert_eq!(configuration.contents.get(key), Some(&value));

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_profile_array_value_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Set up event listener to capture the emitted event
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Update configuration with array value
//     let key = "arrayKey";
//     let value = json!(["item1", "item2", 42, true, {"nested": "object"}]);
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.to_string(),
//                     value: value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(received_event_data.affected_keys, vec![key.to_string()]);
//     assert_eq!(received_event_data.changes.get(key), Some(&value));

//     // Verify configuration was updated
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     assert!(configuration.keys.contains(&key.to_string()));
//     assert_eq!(configuration.contents.get(key), Some(&value));

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_profile_overwrite_existing_key() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     let key = "overwriteKey";

//     // First update with initial value
//     let initial_value = JsonValue::String("initial".to_string());
//     let first_update = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.to_string(),
//                     value: initial_value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(first_update.is_ok());

//     // Verify initial value is set
//     let app_description_after_first = app.describe_app(&ctx).await.unwrap();
//     let configuration_after_first = &app_description_after_first.configuration;
//     assert_eq!(
//         configuration_after_first.contents.get(key),
//         Some(&initial_value)
//     );

//     // Set up event listener for the second update
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Update with new value
//     let new_value = JsonValue::String("updated".to_string());
//     let second_update = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.to_string(),
//                     value: new_value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(second_update.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(received_event_data.affected_keys, vec![key.to_string()]);
//     assert_eq!(received_event_data.changes.get(key), Some(&new_value));

//     // Verify value was overwritten
//     let app_description_after_second = app.describe_app(&ctx).await.unwrap();
//     let configuration_after_second = &app_description_after_second.configuration;
//     assert_eq!(
//         configuration_after_second.contents.get(key),
//         Some(&new_value)
//     );
//     assert_ne!(
//         configuration_after_second.contents.get(key),
//         Some(&initial_value)
//     );

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_builtin_keys() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Set up event listener for colorTheme update
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Test updating built-in colorTheme key
//     let color_theme_key = "colorTheme";
//     let new_theme_value = JsonValue::String("moss.sapic-theme.darkDefault".to_string());
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: color_theme_key.to_string(),
//                     value: new_theme_value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(
//         received_event_data.affected_keys,
//         vec![color_theme_key.to_string()]
//     );
//     assert_eq!(
//         received_event_data.changes.get(color_theme_key),
//         Some(&new_theme_value)
//     );

//     // Verify colorTheme was updated
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     assert_eq!(
//         configuration.contents.get(color_theme_key),
//         Some(&new_theme_value)
//     );

//     // Set up event listener for languages update
//     let event_received_2 = Arc::new(AtomicBool::new(false));
//     let received_data_2 = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_2_clone = event_received_2.clone();
//     let received_data_2_clone = received_data_2.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_2_clone.clone();
//             let received_data = received_data_2_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Test updating built-in language key
//     let languages_key = "language";
//     let new_language_value = JsonValue::String("de".to_string());
//     let locale_update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: languages_key.to_string(),
//                     value: new_language_value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(locale_update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the second event was received
//     assert!(
//         event_received_2.load(Ordering::SeqCst),
//         "Event should have been emitted for language"
//     );

//     // Verify the second event data
//     let received_event_data_2 = received_data_2.lock().unwrap().take().unwrap();
//     assert_eq!(
//         received_event_data_2.affected_keys,
//         vec![languages_key.to_string()]
//     );
//     assert_eq!(
//         received_event_data_2.changes.get(languages_key),
//         Some(&new_language_value)
//     );

//     // Verify language was updated
//     let final_app_description = app.describe_app(&ctx).await.unwrap();
//     let final_configuration = &final_app_description.configuration;
//     assert_eq!(
//         final_configuration.contents.get(languages_key),
//         Some(&new_language_value)
//     );

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_multiple_keys() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Update multiple configuration keys
//     let keys_and_values = vec![
//         ("key1", JsonValue::String("value1".to_string())),
//         ("key2", JsonValue::Number(serde_json::Number::from(100))),
//         ("key3", JsonValue::Bool(false)),
//         ("key4", json!({"nested": "data"})),
//     ];

//     // Process each key-value pair individually with its own event listener
//     for (key, value) in &keys_and_values {
//         // Set up event listener for this specific key update
//         let event_received = Arc::new(AtomicBool::new(false));
//         let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//         let event_received_clone = event_received.clone();
//         let received_data_clone = received_data.clone();

//         app.handle()
//             .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//                 let event_received = event_received_clone.clone();
//                 let received_data = received_data_clone.clone();

//                 if let Ok(payload) =
//                     serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//                 {
//                     event_received.store(true, Ordering::SeqCst);
//                     if let Ok(mut data) = received_data.lock() {
//                         *data = Some(payload);
//                     }
//                 }
//             });

//         let update_result = app
//             .update_configuration(
//                 &ctx,
//                 &app_delegate,
//                 UpdateConfigurationInput {
//                     inner: UpdateConfigurationParams {
//                         key: key.to_string(),
//                         value: value.clone(),
//                         target: ConfigurationTarget::Profile,
//                     },
//                 },
//             )
//             .await;
//         assert!(update_result.is_ok());

//         // Give some time for the event to be processed
//         tokio::time::sleep(Duration::from_millis(100)).await;

//         // Verify this specific event was received and data is correct
//         assert!(
//             event_received.load(Ordering::SeqCst),
//             "Event should have been emitted for key: {}",
//             key
//         );

//         let received_event_data = received_data.lock().unwrap().take().unwrap();
//         assert_eq!(received_event_data.affected_keys, vec![key.to_string()]);
//         assert_eq!(received_event_data.changes.get(*key), Some(value));
//     }

//     // Verify all keys were updated
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     for (key, expected_value) in keys_and_values {
//         assert!(configuration.keys.contains(&key.to_string()));
//         assert_eq!(configuration.contents.get(key), Some(&expected_value));
//     }

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_empty_key_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Set up event listener
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Try to update configuration with empty key (should succeed since no validation exists)
//     let key = String::new(); // Empty key
//     let value = JsonValue::String("value".to_string());
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.clone(),
//                     value: value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;

//     // Should succeed since empty key validation is not implemented
//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(received_event_data.affected_keys, vec![key.clone()]);
//     assert_eq!(received_event_data.changes.get(&key), Some(&value));

//     // Verify that the empty key was actually set
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     assert!(configuration.keys.contains(&"".to_string()));
//     assert_eq!(
//         configuration.contents.get(""),
//         Some(&JsonValue::String("value".to_string()))
//     );

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_null_value_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Set up event listener
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Update configuration with null value
//     let key = "nullKey";
//     let value = JsonValue::Null;
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.to_string(),
//                     value: value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;

//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(received_event_data.affected_keys, vec![key.to_string()]);
//     assert_eq!(received_event_data.changes.get(key), Some(&value));

//     // Verify configuration was updated with null value
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     assert!(configuration.keys.contains(&key.to_string()));
//     assert_eq!(configuration.contents.get(key), Some(&value));

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_default_values_preserved() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Check that default values are present before any updates
//     let initial_app_description = app.describe_app(&ctx).await.unwrap();
//     let initial_configuration = &initial_app_description.configuration;

//     // Verify default colorTheme and language are present
//     assert!(
//         initial_configuration
//             .keys
//             .contains(&"colorTheme".to_string())
//     );
//     assert!(initial_configuration.keys.contains(&"language".to_string()));
//     assert_eq!(
//         initial_configuration.contents.get("colorTheme"),
//         Some(&JsonValue::String(
//             "moss.sapic-theme.lightDefault".to_string()
//         ))
//     );

//     assert_eq!(
//         initial_configuration.contents.get("language"),
//         Some(&JsonValue::String("en".to_string()))
//     );

//     // Set up event listener
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Update a custom key
//     let custom_key = "customKey";
//     let custom_value = JsonValue::String("customValue".to_string());
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: custom_key.to_string(),
//                     value: custom_value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;
//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(
//         received_event_data.affected_keys,
//         vec![custom_key.to_string()]
//     );
//     assert_eq!(
//         received_event_data.changes.get(custom_key),
//         Some(&custom_value)
//     );

//     // Verify that default values are still present along with the new custom key
//     let final_app_description = app.describe_app(&ctx).await.unwrap();
//     let final_configuration = &final_app_description.configuration;

//     // Default values should still be there
//     assert!(final_configuration.keys.contains(&"colorTheme".to_string()));
//     assert!(final_configuration.keys.contains(&"language".to_string()));
//     assert_eq!(
//         final_configuration.contents.get("colorTheme"),
//         Some(&JsonValue::String(
//             "moss.sapic-theme.lightDefault".to_string()
//         ))
//     );
//     assert_eq!(
//         final_configuration.contents.get("language"),
//         Some(&JsonValue::String("en".to_string()))
//     );

//     // Custom key should also be present
//     assert!(final_configuration.keys.contains(&custom_key.to_string()));
//     assert_eq!(
//         final_configuration.contents.get(custom_key),
//         Some(&custom_value)
//     );

//     cleanup().await;
// }

// #[tokio::test]
// async fn update_configuration_event_emission_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Set up event listener
//     let event_received = Arc::new(AtomicBool::new(false));
//     let received_data = Arc::new(Mutex::new(None::<OnDidChangeConfigurationForFrontend>));

//     let event_received_clone = event_received.clone();
//     let received_data_clone = received_data.clone();

//     app.handle()
//         .listen(ON_DID_CHANGE_CONFIGURATION_CHANNEL, move |event| {
//             let event_received = event_received_clone.clone();
//             let received_data = received_data_clone.clone();

//             if let Ok(payload) =
//                 serde_json::from_str::<OnDidChangeConfigurationForFrontend>(event.payload())
//             {
//                 event_received.store(true, Ordering::SeqCst);
//                 if let Ok(mut data) = received_data.lock() {
//                     *data = Some(payload);
//                 }
//             }
//         });

//     // Update configuration - this should emit an event
//     let key = "eventTestKey";
//     let value = JsonValue::String("eventTestValue".to_string());
//     let update_result = app
//         .update_configuration(
//             &ctx,
//             &app_delegate,
//             UpdateConfigurationInput {
//                 inner: UpdateConfigurationParams {
//                     key: key.to_string(),
//                     value: value.clone(),
//                     target: ConfigurationTarget::Profile,
//                 },
//             },
//         )
//         .await;

//     // The main test is that the update succeeds without error
//     assert!(update_result.is_ok());

//     // Give some time for the event to be processed
//     tokio::time::sleep(Duration::from_millis(100)).await;

//     // Verify that the event was received
//     assert!(
//         event_received.load(Ordering::SeqCst),
//         "Event should have been emitted"
//     );

//     // Verify the event data
//     let received_event_data = received_data.lock().unwrap().take().unwrap();
//     assert_eq!(received_event_data.affected_keys, vec![key.to_string()]);
//     assert_eq!(received_event_data.changes.get(key), Some(&value));

//     // Verify the configuration was actually updated
//     let app_description = app.describe_app(&ctx).await.unwrap();
//     let configuration = &app_description.configuration;
//     assert!(configuration.keys.contains(&key.to_string()));
//     assert_eq!(configuration.contents.get(key), Some(&value));

//     cleanup().await;
// }
