mod shared;

// use moss_environment::{
//     environment::{Environment, EnvironmentError},
//     models::{
//         file::{EnvironmentFile, EnvironmentFileVariable},
//         types::{VariableKind, VariableValue},
//     },
// };
// use moss_fs::RealFileSystem;
// use shared::test_environment_data;
// use std::{collections::HashMap, sync::Arc};

// #[tokio::test]
// async fn test_create_environment_with_empty_file() {
//     let environment_file_path = test_environment_data();
//     let new_environment_result = Environment::new(
//         environment_file_path.clone(),
//         Arc::new(RealFileSystem::new()),
//     )
//     .await;
//     assert!(new_environment_result.is_ok());

//     let environment = new_environment_result.unwrap();
//     let variables = environment.variables();
//     assert_eq!(variables.read().await.len(), 0);

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_create_environment_with_malformed_json() {
//     let environment_file_path = test_environment_data();

//     // Write malformed JSON
//     tokio::fs::write(
//         &environment_file_path,
//         r#"{"values": {"test": {"kind": "default", "value": "test }}"#,
//     )
//     .await
//     .unwrap();

//     // Attempt to create environment should fail with JSON parse error
//     let fs = Arc::new(RealFileSystem::new());
//     let new_environment_result = Environment::new(environment_file_path.clone(), fs).await;
//     assert!(matches!(
//         new_environment_result.unwrap_err(),
//         EnvironmentError::JsonParseError(_)
//     ));

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_create_environment_with_invalid_variable_type() {
//     let environment_file_path = test_environment_data();

//     // Write JSON with invalid variable type
//     let invalid_json = r#"{
//         "values": {
//             "test": {
//                 "kind": "invalid_kind",
//                 "value": "test",
//                 "desc": null
//             }
//         }
//     }"#;
//     tokio::fs::write(&environment_file_path, invalid_json)
//         .await
//         .unwrap();

//     // Attempt to create environment should fail with JSON parse error
//     let fs = Arc::new(RealFileSystem::new());
//     let new_environment_result = Environment::new(environment_file_path.clone(), fs).await;
//     assert!(matches!(
//         new_environment_result.unwrap_err(),
//         EnvironmentError::JsonParseError(_)
//     ));

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_create_environment_with_invalid_variable_value() {
//     let environment_file_path = test_environment_data();

//     // Write JSON with invalid variable value type
//     let invalid_json = r#"{
//         "values": {
//             "test": {
//                 "kind": "default",
//                 "value": {"invalid": "value"},
//                 "desc": null
//             }
//         }
//     }"#;
//     tokio::fs::write(&environment_file_path, invalid_json)
//         .await
//         .unwrap();

//     // Attempt to create environment should fail with JSON parse error
//     let fs = Arc::new(RealFileSystem::new());
//     let new_environment_result = Environment::new(environment_file_path.clone(), fs).await;
//     assert!(matches!(
//         new_environment_result.unwrap_err(),
//         EnvironmentError::JsonParseError(_)
//     ));

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_create_environment_with_duplicate_variables() {
//     let environment_file_path = test_environment_data();

//     // Write JSON with duplicate variable names
//     let invalid_json = r#"{
//         "values": {
//             "test": {
//                 "kind": "default",
//                 "value": "test1",
//                 "desc": null
//             },
//             "test": {
//                 "kind": "default",
//                 "value": "test2",
//                 "desc": null
//             }
//         }
//     }"#;
//     tokio::fs::write(&environment_file_path, invalid_json)
//         .await
//         .unwrap();

//     let new_environment_result = Environment::new(
//         environment_file_path.clone(),
//         Arc::new(RealFileSystem::new()),
//     )
//     .await;
//     assert!(new_environment_result.is_ok());

//     let environment = new_environment_result.unwrap();
//     let variables = environment.variables();
//     assert_eq!(variables.read().await.len(), 1);

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_create_environment_with_large_file() {
//     let environment_file_path = test_environment_data();

//     // Create a large number of variables
//     let mut test_vars = HashMap::new();
//     for i in 0..1000 {
//         test_vars.insert(
//             format!("var_{}", i),
//             EnvironmentFileVariable {
//                 kind: VariableKind::Default,
//                 value: VariableValue::String(format!("value_{}", i)),
//                 desc: Some(format!("Description for var_{}", i)),
//             },
//         );
//     }

//     // Write test variables to file
//     let file_content = serde_json::to_string(&EnvironmentFile {
//         values: test_vars.clone(),
//     })
//     .unwrap();
//     tokio::fs::write(&environment_file_path, file_content)
//         .await
//         .unwrap();

//     // Create new environment instance
//     let fs = Arc::new(RealFileSystem::new());
//     let new_environment_result = Environment::new(
//         environment_file_path.clone(),
//         Arc::new(RealFileSystem::new()),
//     )
//     .await;

//     assert!(new_environment_result.is_ok());

//     let environment = new_environment_result.unwrap();
//     let variables = environment.variables().read().await;
//     assert_eq!(variables.len(), 1000);

//     // Verify a few random variables
//     for i in 0..10 {
//         let var_name = format!("var_{}", i);
//         let var = variables.get(&var_name).unwrap();
//         assert_eq!(var.kind, VariableKind::Default);
//         assert_eq!(var.value, VariableValue::String(format!("value_{}", i)));
//         assert_eq!(var.desc, Some(format!("Description for var_{}", i)));
//     }

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_create_environment_with_special_characters() {
//     let environment_file_path = test_environment_data();

//     // Create variables with special characters in names and values
//     let mut test_vars = HashMap::new();
//     test_vars.insert(
//         "var@#$%".to_string(),
//         EnvironmentFileVariable {
//             kind: VariableKind::Default,
//             value: VariableValue::String("value@#$%".to_string()),
//             desc: Some("Description with @#$%".to_string()),
//         },
//     );
//     test_vars.insert(
//         "var_unicode_测试".to_string(),
//         EnvironmentFileVariable {
//             kind: VariableKind::Default,
//             value: VariableValue::String("value_unicode_测试".to_string()),
//             desc: Some("Description with unicode 测试".to_string()),
//         },
//     );

//     // Write test variables to file
//     let file_content = serde_json::to_string(&EnvironmentFile {
//         values: test_vars.clone(),
//     })
//     .unwrap();
//     tokio::fs::write(&environment_file_path, file_content)
//         .await
//         .unwrap();

//     // Create new environment instance
//     let fs = Arc::new(RealFileSystem::new());
//     let new_environment_result = Environment::new(
//         environment_file_path.clone(),
//         Arc::new(RealFileSystem::new()),
//     )
//     .await;

//     assert!(new_environment_result.is_ok());

//     let environment = new_environment_result.unwrap();
//     let variables = environment.variables().read().await;
//     assert_eq!(variables.len(), 2);

//     let special_char_var = variables.get("var@#$%").unwrap();
//     assert_eq!(
//         special_char_var.value,
//         VariableValue::String("value@#$%".to_string())
//     );

//     let unicode_var = variables.get("var_unicode_测试").unwrap();
//     assert_eq!(
//         unicode_var.value,
//         VariableValue::String("value_unicode_测试".to_string())
//     );

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_environment_persistence() {
//     let environment_file_path = test_environment_data();

//     {
//         // Create test variables
//         let mut test_vars = HashMap::new();
//         test_vars.insert(
//             "test_string".to_string(),
//             EnvironmentFileVariable {
//                 kind: VariableKind::Default,
//                 value: VariableValue::String("test_value".to_string()),
//                 desc: None,
//             },
//         );
//         test_vars.insert(
//             "test_number".to_string(),
//             EnvironmentFileVariable {
//                 kind: VariableKind::Default,
//                 value: VariableValue::Number(serde_json::Number::from(42)),
//                 desc: None,
//             },
//         );
//         test_vars.insert(
//             "test_boolean".to_string(),
//             EnvironmentFileVariable {
//                 kind: VariableKind::Secret,
//                 value: VariableValue::Boolean(true),
//                 desc: None,
//             },
//         );

//         // Write test variables to file
//         let file_content = serde_json::to_string(&EnvironmentFile {
//             values: test_vars.clone(),
//         })
//         .unwrap();
//         tokio::fs::write(&environment_file_path, file_content)
//             .await
//             .unwrap();
//     }

//     let new_environment_result = Environment::new(
//         environment_file_path.clone(),
//         Arc::new(RealFileSystem::new()),
//     )
//     .await;

//     assert!(new_environment_result.is_ok());

//     let environment = new_environment_result.unwrap();
//     let variables = environment.variables();
//     let vars = variables.read().await;

//     // Verify variables
//     assert_eq!(vars.len(), 3);
//     assert_eq!(
//         vars.get("test_string").unwrap().value,
//         VariableValue::String("test_value".to_string())
//     );
//     assert_eq!(
//         vars.get("test_number").unwrap().value,
//         VariableValue::Number(serde_json::Number::from(42))
//     );
//     assert_eq!(
//         vars.get("test_boolean").unwrap().value,
//         VariableValue::Boolean(true)
//     );
//     assert_eq!(vars.get("test_boolean").unwrap().kind, VariableKind::Secret);

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }
