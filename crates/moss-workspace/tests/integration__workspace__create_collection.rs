use moss_workspace::models::operations::CreateCollectionInput;
use moss_workspace::models::types::CollectionInfo;
use moss_workspace::sanitizer::encode_directory_name;
use moss_workspace::workspace::OperationError;
use crate::shared::{random_collection_name, setup_test_workspace, SPECIAL_CHARS};

mod shared;

#[tokio::test]
async fn create_collection_success() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let expected_path = workspace_path.join(&collection_name);
    let output = workspace.create_collection(
        CreateCollectionInput {
            name: collection_name.clone()
        }
    ).await.unwrap();

    assert!(expected_path.exists());

    // Check updating collections
    let collections = workspace.list_collections().await.unwrap();
    assert_eq!(collections.0.len(), 1);
    assert_eq!(collections.0[0].key, output.key);
    assert_eq!(collections.0[0].name, collection_name);

    // Clean up
    {
        std::fs::remove_dir_all(workspace_path).unwrap()
    }

}

#[tokio::test]
async fn create_collection_empty_name() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = "".to_string();
    let result = workspace.create_collection(
        CreateCollectionInput {
            name: collection_name.clone()
        }
    ).await;
    assert!(matches!(result, Err(OperationError::Validation(_))));

    // Clean up
    {
        std::fs::remove_dir_all(workspace_path).unwrap()
    }

}

#[tokio::test]
async fn create_collection_already_exists() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    workspace.create_collection(
        CreateCollectionInput {
            name: collection_name.clone()
        }
    ).await.unwrap();

    let result = workspace.create_collection(
        CreateCollectionInput {
            name: collection_name.clone()
        }
    ).await;

    assert!(matches!(result, Err(OperationError::AlreadyExists {..})));
    {
        std::fs::remove_dir_all(workspace_path).unwrap()
    }

}

#[tokio::test]
async fn create_collection_special_chars() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name_list = SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_collection_name()))
        .collect::<Vec<String>>();

    for name in collection_name_list {
        let expected_path = workspace_path.join(encode_directory_name(&name));
        let output = workspace.create_collection(
            CreateCollectionInput {
                name: name.clone()
            }
        ).await.unwrap();

        assert!(expected_path.exists());

        // Check updating collections
        let collections = workspace.list_collections().await.unwrap();
        assert!(collections.0.iter().any(|info| info == &CollectionInfo {
            key: output.key,
            name: name.clone(),
            order: None,
        }));
    }

    // Clean up
    {
        std::fs::remove_dir_all(workspace_path).unwrap();
    }
}