use moss_workspace::workspace::OperationError;
use moss_workspace::models::operations::{CreateCollectionInput, RenameCollectionInput};
use moss_workspace::models::types::CollectionInfo;
use moss_workspace::sanitizer::encode_directory_name;
use crate::shared::{random_collection_name, random_workspace_name, setup_test_workspace, SPECIAL_CHARS};

mod shared;

#[tokio::test]
async fn rename_collection_success() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let old_path = workspace_path.join(&old_collection_name);
    let key = workspace.create_collection(
        CreateCollectionInput {
            name: old_collection_name.clone()
        }
    ).await.unwrap().key;

    let new_collection_name = random_collection_name();
    let result = workspace.rename_collection(
        RenameCollectionInput {
            key,
            new_name: new_collection_name.clone()
        }
    ).await;
    assert!(result.is_ok());

    // Check filesystem rename
    let expected_path = workspace_path.join(&new_collection_name);
    assert!(expected_path.exists());
    assert!(!old_path.exists());

    // Check updating collections
    let collections = workspace.list_collections().await.unwrap();
    assert_eq!(collections.0.len(), 1);
    assert_eq!(collections.0[0], CollectionInfo {
        key,
        name: new_collection_name,
        order: None,
    });

    {
        std::fs::remove_dir_all(workspace_path).unwrap();
    }
}

#[tokio::test]
async fn rename_collection_empty_name() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let key = workspace.create_collection(
        CreateCollectionInput {
            name: old_collection_name.clone()
        }
    ).await.unwrap().key;

    let new_collection_name = "".to_string();
    let result = workspace.rename_collection(
        RenameCollectionInput {
            key,
            new_name: new_collection_name.clone()
        }
    ).await;

    assert!(matches!(result, Err(OperationError::Validation(_))));

    {
        std::fs::remove_dir_all(workspace_path).unwrap();
    }
}

#[tokio::test]
async fn rename_collection_unchanged() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let key = workspace.create_collection(
        CreateCollectionInput {
            name: old_collection_name.clone()
        }
    ).await.unwrap().key;

    let new_collection_name = old_collection_name;
    let result = workspace.rename_collection(
        RenameCollectionInput {
            key,
            new_name: new_collection_name.clone()
        }
    ).await;

    {
        std::fs::remove_dir_all(workspace_path).unwrap()
    }
}

#[tokio::test]
async fn rename_collection_already_exists() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let existing_collection_name = random_collection_name();

    // Create an existing collection
    workspace.create_collection(
        CreateCollectionInput {
            name: existing_collection_name.clone()
        }
    ).await.unwrap();

    let new_collection_name = random_collection_name();
    // Create a collection to test renaming
    let key = workspace.create_collection(
        CreateCollectionInput {
            name: new_collection_name.clone()
        }
    ).await.unwrap().key;

    // Try renaming the new collection to an existing collection name
    let result = workspace.rename_collection(
        RenameCollectionInput {
            key,
            new_name: existing_collection_name.clone()
        }
    ).await;
    assert!(matches!(result, Err(OperationError::AlreadyExists {..})));

    // Clean up
    {
        std::fs::remove_dir_all(workspace_path).unwrap();
    }
}

#[tokio::test]
async fn rename_collection_special_chars() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let key = workspace.create_collection(
        CreateCollectionInput {
            name: collection_name.clone()
        }
    ).await.unwrap().key;

    for char in SPECIAL_CHARS {
        let new_collection_name = format!("{collection_name}{char}");
        let expected_path = workspace_path.join(encode_directory_name(&new_collection_name));
        workspace.rename_collection(
            RenameCollectionInput {
                key,
                new_name: new_collection_name.clone()
            }
        ).await.unwrap();

        // Checking updating collections
        let collections = workspace.list_collections().await.unwrap();
        assert_eq!(collections.0.len(), 1);
        assert_eq!(collections.0[0], CollectionInfo {
            key,
            name: new_collection_name.clone(),
            order: None,
        });
    }

    {
        std::fs::remove_dir_all(workspace_path).unwrap();
    }
}