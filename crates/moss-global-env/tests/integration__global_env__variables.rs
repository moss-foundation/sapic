// use anyhow::Result;
// use moss_fs::RealFileSystem;
// use moss_global_env::{
//     manager::Environment,
//     models::{
//         file::{EnvironmentFile, EnvironmentFileVariable},
//         types::{VariableKind, VariableRawValue},
//     },
// };
// use std::{collections::HashMap, path::PathBuf, sync::Arc};

// pub fn random_string(length: usize) -> String {
//     use rand::{distr::Alphanumeric, Rng};

//     rand::rng()
//         .sample_iter(Alphanumeric)
//         .take(length)
//         .map(char::from)
//         .collect()
// }

// #[tokio::test]
// async fn test_empty_environment() {
//     let base_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//         .join("tests")
//         .join("data");
//     let environment_name = format!("Test_{}_Environment.json", random_string(10));
//     let environment_file_path = base_path.join(environment_name);

//     let environment = Environment::new(
//         environment_file_path.clone(),
//         Arc::new(RealFileSystem::new()),
//     )
//     .await
//     .unwrap();

//     let variables = environment.variables().await.unwrap();
//     assert_eq!(variables.read().await.len(), 0);

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_environment_persistence() {
//     let base_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//         .join("tests")
//         .join("data");
//     let environment_name = format!("Test_{}_Environment.json", random_string(10));
//     let environment_file_path = base_path.join(environment_name);

//     {
//         // Create test variables
//         let mut test_vars = HashMap::new();
//         test_vars.insert(
//             "test_string".to_string(),
//             EnvironmentFileVariable {
//                 kind: VariableKind::Default,
//                 value: VariableRawValue::String("test_value".to_string()),
//                 desc: None,
//             },
//         );
//         test_vars.insert(
//             "test_number".to_string(),
//             EnvironmentFileVariable {
//                 kind: VariableKind::Default,
//                 value: VariableRawValue::Number(serde_json::Number::from(42)),
//                 desc: None,
//             },
//         );
//         test_vars.insert(
//             "test_boolean".to_string(),
//             EnvironmentFileVariable {
//                 kind: VariableKind::Secret,
//                 value: VariableRawValue::Boolean(true),
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

//     let environment = Environment::new(
//         environment_file_path.clone(),
//         Arc::new(RealFileSystem::new()),
//     )
//     .await
//     .unwrap();

//     let variables = environment.variables().await.unwrap();
//     let vars = variables.read().await;

//     // Verify variables
//     assert_eq!(vars.len(), 3);
//     assert_eq!(
//         vars.get("test_string").unwrap().value,
//         VariableRawValue::String("test_value".to_string())
//     );
//     assert_eq!(
//         vars.get("test_number").unwrap().value,
//         VariableRawValue::Number(serde_json::Number::from(42))
//     );
//     assert_eq!(
//         vars.get("test_boolean").unwrap().value,
//         VariableRawValue::Boolean(true)
//     );
//     assert_eq!(vars.get("test_boolean").unwrap().kind, VariableKind::Secret);

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_invalid_json_file() {
//     let base_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//         .join("tests")
//         .join("data");
//     let environment_name = format!("Test_{}_Environment.json", random_string(10));
//     let environment_file_path = base_path.join(environment_name);

//     // Write invalid JSON to file
//     tokio::fs::write(&environment_file_path, "invalid json content")
//         .await
//         .unwrap();

//     let environment = Environment::new(
//         environment_file_path.clone(),
//         Arc::new(RealFileSystem::new()),
//     )
//     .await
//     .unwrap();

//     // Attempt to read variables should fail
//     let result = environment.variables().await;
//     assert!(result.is_err());

//     // Clean up
//     {
//         tokio::fs::remove_file(environment_file_path).await.unwrap();
//     }
// }
