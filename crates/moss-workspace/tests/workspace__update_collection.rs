#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_testutils::random_name::random_collection_name;
use moss_workspace::models::{
    operations::{CreateCollectionInput, UpdateCollectionInput},
    primitives::CollectionId,
};

use crate::shared::{generate_random_icon, setup_test_workspace};

#[tokio::test]
async fn rename_collection_success() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: old_collection_name.clone(),
                order: 0,
                external_path: None,
                repository: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();

    let new_collection_name = random_collection_name();
    let _ = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: create_collection_output.id.clone(),
                name: Some(new_collection_name.clone()),
                repository: None,
                icon_path: None,
                order: None,
                pinned: None,
                expanded: None,
            },
        )
        .await
        .unwrap();

    // Verify the manifest is updated
    let collection = workspace
        .collection(&create_collection_output.id.into())
        .await
        .unwrap();
    assert_eq!(
        collection.describe().await.unwrap().name,
        new_collection_name
    );

    cleanup().await;
}

#[tokio::test]
async fn rename_collection_empty_name() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: old_collection_name.clone(),
                order: 0,
                external_path: None,
                repository: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();

    let new_collection_name = "".to_string();
    let rename_collection_result = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: create_collection_output.id,
                name: Some(new_collection_name.clone()),
                repository: None,
                icon_path: None,
                order: None,
                pinned: None,
                expanded: None,
            },
        )
        .await;

    assert!(rename_collection_result.is_err());
    cleanup().await;
}

#[tokio::test]
async fn rename_collection_unchanged() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let old_collection_name = random_collection_name();
    let create_collection_output = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: old_collection_name.clone(),
                order: 0,
                external_path: None,
                repository: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();

    let new_collection_name = old_collection_name;
    let _ = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: create_collection_output.id,
                name: Some(new_collection_name),
                repository: None,
                icon_path: None,
                order: None,
                pinned: None,
                expanded: None,
            },
        )
        .await
        .unwrap();

    cleanup().await;
}

#[tokio::test]
async fn rename_collection_nonexistent_id() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    // Use a random ID that doesn't exist
    let nonexistent_id = CollectionId::new();

    let result = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: nonexistent_id,
                name: Some(random_collection_name()),
                repository: None,
                icon_path: None,
                order: None,
                pinned: None,
                expanded: None,
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn update_collection_new_icon() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;
    let collection_name = random_collection_name();
    let id = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.to_string(),
                order: 0,
                external_path: None,
                repository: None,
                icon_path: None,
            },
        )
        .await
        .unwrap()
        .id;

    let icon_path = workspace.abs_path().join("test_icon.png");
    generate_random_icon(&icon_path);

    let _ = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: id.clone(),
                name: None,
                repository: None,
                icon_path: Some(ChangePath::Update(icon_path.clone())),
                order: None,
                pinned: None,
                expanded: None,
            },
        )
        .await
        .unwrap();

    // Verify the icon is generated
    let collection = workspace.collection(&id).await.unwrap();
    assert!(collection.icon_path().is_some());

    cleanup().await;
}

#[tokio::test]
async fn update_collection_remove_icon() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;
    let collection_name = random_collection_name();

    let icon_path = workspace.abs_path().join("test_icon.png");
    generate_random_icon(&icon_path);

    let id = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: 0,
                external_path: None,
                repository: None,
                icon_path: Some(icon_path.clone()),
            },
        )
        .await
        .unwrap()
        .id;

    let _ = workspace
        .update_collection(
            &ctx,
            UpdateCollectionInput {
                id: id.clone(),
                name: None,
                repository: None,
                icon_path: Some(ChangePath::Remove),
                order: None,
                pinned: None,
                expanded: None,
            },
        )
        .await
        .unwrap();

    // Verify the icon is removed
    let collection = workspace.collection(&id).await.unwrap();
    assert!(collection.icon_path().is_none());

    cleanup().await;
}

// TODO: Reenable this test once we introduce relinking a collection with a new remote repo

// #[tokio::test]
// async fn update_collection_repo() {
//     let (ctx, workspace, cleanup) = setup_test_workspace().await;
//
//     let collection_name = random_collection_name();
//     let old_repo = "https://github.com/xxx/1.git".to_string();
//     let new_repo = "https://github.com/xxx/2.git".to_string();
//     let new_normalized_repo = "github.com/xxx/2";
//     let create_collection_output = workspace
//         .create_collection(
//             &ctx,
//             &CreateCollectionInput {
//                 name: collection_name,
//                 order: 0,
//                 external_path: None,
//                 repository: Some(old_repo),
//                 icon_path: None,
//             },
//         )
//         .await
//         .unwrap();
//
//     let _ = workspace
//         .update_collection(
//             &ctx,
//             UpdateCollectionInput {
//                 id: create_collection_output.id.clone(),
//                 name: None,
//                 repository: Some(ChangeString::Update(new_repo.clone())),
//                 icon_path: None,
//                 order: None,
//                 pinned: None,
//                 expanded: None,
//             },
//         )
//         .await
//         .unwrap();
//
//     // Verify the manifest is updated
//     let collection = workspace
//         .collection(&create_collection_output.id.into())
//         .await
//         .unwrap();
//
//     assert_eq!(
//         collection.describe().await.unwrap().repository,
//         Some(new_normalized_repo.to_owned())
//     );
//
//     cleanup().await;
// }
