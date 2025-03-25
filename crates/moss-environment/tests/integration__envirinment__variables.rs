use anyhow::Result;
use moss_environment::{
    environment::Environment,
    models::{
        file::EnvironmentFile,
        types::{VariableInfo, VariableKind, VariableValue},
    },
};
use moss_fs::RealFileSystem;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub fn random_string(length: usize) -> String {
    use rand::{distr::Alphanumeric, Rng};

    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

async fn create_test_environment() -> Result<(Environment, PathBuf)> {
    let base_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data");
    let environment_name = format!("Test_{}_Environment.json", random_string(10));
    let environment_path = base_path.join(environment_name);

    let environment =
        Environment::new(environment_path.clone(), Arc::new(RealFileSystem::new())).await?;

    Ok((environment, environment_path))
}

#[tokio::test]
async fn test_empty_environment() {
    let base_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data");
    let environment_name = format!("Test_{}_Environment.json", random_string(10));
    let environment_file_path = base_path.join(environment_name);

    let environment = Environment::new(
        environment_file_path.clone(),
        Arc::new(RealFileSystem::new()),
    )
    .await
    .unwrap();

    let variables = environment.variables().await.unwrap();
    assert_eq!(variables.read().await.len(), 0);

    // Clean up
    {
        tokio::fs::remove_file(environment_file_path).await.unwrap();
    }
}

#[tokio::test]
async fn test_environment_with_variables() {
    let base_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data");
    let environment_name = format!("Test_{}_Environment.json", random_string(10));
    let environment_file_path = base_path.join(environment_name);

    {
        // Create test variables
        let mut test_vars = HashMap::new();
        test_vars.insert(
            "test_string".to_string(),
            VariableInfo {
                kind: VariableKind::Default,
                value: VariableValue::String("test_value".to_string()),
            },
        );
        test_vars.insert(
            "test_number".to_string(),
            VariableInfo {
                kind: VariableKind::Default,
                value: VariableValue::Number(serde_json::Number::from(42)),
            },
        );
        test_vars.insert(
            "test_boolean".to_string(),
            VariableInfo {
                kind: VariableKind::Secret,
                value: VariableValue::Boolean(true),
            },
        );

        // Write test variables to file
        let file_content = serde_json::to_string(&EnvironmentFile {
            values: test_vars.clone(),
        })
        .unwrap();
        tokio::fs::write(&environment_file_path, file_content)
            .await
            .unwrap();
    }

    let environment = Environment::new(
        environment_file_path.clone(),
        Arc::new(RealFileSystem::new()),
    )
    .await
    .unwrap();

    let variables = environment.variables().await.unwrap();
    let vars = variables.read().await;

    // Verify variables
    assert_eq!(vars.len(), 3);
    assert_eq!(
        vars.get("test_string").unwrap().value,
        VariableValue::String("test_value".to_string())
    );
    assert_eq!(
        vars.get("test_number").unwrap().value,
        VariableValue::Number(serde_json::Number::from(42))
    );
    assert_eq!(
        vars.get("test_boolean").unwrap().value,
        VariableValue::Boolean(true)
    );
    assert_eq!(vars.get("test_boolean").unwrap().kind, VariableKind::Secret);

    // Clean up
    // {
    //     tokio::fs::remove_file(environment_file_path).await.unwrap();
    // }
}
