use crate::shared::{random_collection_name, setup_test_workspace, SPECIAL_CHARS};
use moss_workspace::models::operations::CreateCollectionInput;
use moss_workspace::models::types::CollectionInfo;
use moss_workspace::workspace::OperationError;

mod shared;

#[tokio::test]
async fn create_collection_success() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_result = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await;

    assert!(create_collection_result.is_ok());

    let create_collection_output = create_collection_result.unwrap();
    let describe_output = workspace.describe().await.unwrap();

    assert_eq!(describe_output.collections.len(), 1);
    assert_eq!(
        describe_output.collections[0].key,
        create_collection_output.key
    );
    assert_eq!(describe_output.collections[0].name, collection_name);

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_collection_empty_name() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = "".to_string();
    let create_collection_result = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await;

    assert!(matches!(
        create_collection_result,
        Err(OperationError::Validation(_))
    ));

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_collection_already_exists() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await
        .unwrap();

    let create_collection_result = workspace
        .create_collection(CreateCollectionInput {
            name: collection_name.clone(),
        })
        .await;

    assert!(matches!(
        create_collection_result,
        Err(OperationError::AlreadyExists { .. })
    ));

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap()
    }
}

#[tokio::test]
async fn create_collection_special_chars() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let collection_name_list = SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_collection_name()))
        .collect::<Vec<String>>();

    for collection_name in collection_name_list {
        let create_collection_result = workspace
            .create_collection(CreateCollectionInput {
                name: collection_name.clone(),
            })
            .await;
        assert!(create_collection_result.is_ok());

        let create_collection_output = create_collection_result.unwrap();
        let describe_output = workspace.describe().await.unwrap();

        assert!(describe_output.collections.iter().any(|info| info
            == &CollectionInfo {
                key: create_collection_output.key,
                name: collection_name.clone(),
                order: None,
            }));
    }

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}
