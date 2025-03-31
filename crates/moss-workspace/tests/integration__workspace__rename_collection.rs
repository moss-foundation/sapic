use crate::shared::{random_collection_name, setup_test_workspace, SPECIAL_CHARS};
use moss_fs::utils::encode_directory_name;
use moss_workspace::models::operations::{CreateCollectionInput, RenameCollectionInput};
use moss_workspace::models::types::CollectionInfo;
use moss_workspace::workspace::{OperationError, COLLECTIONS_DIR};

mod shared;

#[tokio::test]
async fn rename_collection_success() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: old_collection_name.clone(),
        })
        .await
        .unwrap();

    let new_collection_name = random_collection_name();
    let result = workspace
        .rename_collection(RenameCollectionInput {
            key: create_collection_output.key,
            new_name: new_collection_name.clone(),
        })
        .await;
    assert!(result.is_ok());

    let rename_collection_output = result.unwrap();
    assert!(rename_collection_output.path.exists());
    assert!(!create_collection_output.path.exists());

    // Check updating collections
    let describe_output = workspace.describe().await.unwrap();
    assert_eq!(describe_output.collections.len(), 1);
    assert_eq!(
        describe_output.collections[0],
        CollectionInfo {
            key: create_collection_output.key,
            name: new_collection_name,
            order: None,
        }
    );

    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn rename_collection_empty_name() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: old_collection_name.clone(),
        })
        .await
        .unwrap();

    let new_collection_name = "".to_string();
    let rename_collection_result = workspace
        .rename_collection(RenameCollectionInput {
            key: create_collection_output.key,
            new_name: new_collection_name.clone(),
        })
        .await;

    assert!(matches!(
        rename_collection_result,
        Err(OperationError::Validation(_))
    ));

    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn rename_collection_unchanged() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: old_collection_name.clone(),
        })
        .await
        .unwrap();

    let new_collection_name = old_collection_name;
    let rename_collection_result = workspace
        .rename_collection(RenameCollectionInput {
            key: create_collection_output.key,
            new_name: new_collection_name,
        })
        .await;

    assert!(rename_collection_result.is_ok());

    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn rename_collection_already_exists() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let existing_collection_name = random_collection_name();

    // Create an existing collection
    workspace
        .create_collection(CreateCollectionInput {
            name: existing_collection_name.clone(),
        })
        .await
        .unwrap();

    let new_collection_name = random_collection_name();
    // Create a collection to test renaming
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: new_collection_name.clone(),
        })
        .await
        .unwrap();

    // Try renaming the new collection to an existing collection name
    let result = workspace
        .rename_collection(RenameCollectionInput {
            key: create_collection_output.key,
            new_name: existing_collection_name.clone(),
        })
        .await;
    assert!(matches!(result, Err(OperationError::AlreadyExists { .. })));

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn rename_collection_special_chars() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await
        .unwrap();

    for char in SPECIAL_CHARS {
        let new_collection_name = format!("{collection_name}{char}");
        let expected_path = workspace_path
            .join(COLLECTIONS_DIR)
            .join(encode_directory_name(&new_collection_name));

        let rename_collection_result = workspace
            .rename_collection(RenameCollectionInput {
                key: create_collection_output.key,
                new_name: new_collection_name.clone(),
            })
            .await;
        assert!(rename_collection_result.is_ok());
        assert!(expected_path.exists());

        // Checking updating collections
        let describe_output = workspace.describe().await.unwrap();
        assert_eq!(describe_output.collections.len(), 1);
        assert_eq!(
            describe_output.collections[0],
            CollectionInfo {
                key: create_collection_output.key,
                name: new_collection_name.clone(),
                order: None,
            }
        );
    }

    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}
