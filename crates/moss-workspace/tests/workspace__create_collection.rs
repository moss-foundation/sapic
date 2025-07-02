pub mod shared;

use crate::shared::{collection_key, generate_random_icon, setup_test_workspace};
use moss_collection::{constants::COLLECTION_ICON_FILENAME, dirs::ASSETS_DIR};
use moss_common::api::OperationError;
use moss_storage::storage::operations::{GetItem, ListByPrefix};
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_collection_name};
use moss_workspace::{
    models::operations::CreateCollectionInput,
    storage::entities::collection_store::CollectionCacheEntity,
};

// FIXME: The tests and business logic are poorly organized.
// A collection shouldn’t expose implementation details, and the workspace shouldn’t be
// testing logic that doesn’t belong to it. The DTO for creating a collection should simply
// return the icon path, and in these tests we should check if the icon exists (when expected),
// rather than manually constructing the path where we assume it was saved. With the current
// approach, if the image path logic changes in `moss-collection`, it’ll break tests in
// `moss-workspace`, which clearly shouldn’t happen.

#[tokio::test]
async fn create_collection_success() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: None,
                external_path: None,
                repo: None,
                icon_path: None,
            },
        )
        .await;

    let create_collection_output = create_collection_result.unwrap();
    let collections = workspace.collections(&ctx).await.unwrap();

    assert_eq!(collections.len(), 1);

    // Verify the directory was created
    assert!(create_collection_output.abs_path.exists());

    // Verify the db entry was created
    let id = create_collection_output.id;

    let item_store = workspace.__storage().item_store();
    let entity: CollectionCacheEntity = GetItem::get(item_store.as_ref(), collection_key(id))
        .unwrap()
        .deserialize()
        .unwrap();
    assert_eq!(
        entity,
        CollectionCacheEntity {
            order: None,
            external_abs_path: None
        }
    );

    cleanup().await;
}

#[tokio::test]
async fn create_collection_empty_name() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let collection_name = "".to_string();
    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: None,
                external_path: None,
                repo: None,
                icon_path: None,
            },
        )
        .await;

    assert!(matches!(
        create_collection_result,
        Err(OperationError::InvalidInput(_))
    ));

    // Check that the database is empty
    let item_store = workspace.__storage().item_store();
    let list_result = ListByPrefix::list_by_prefix(item_store.as_ref(), "collection").unwrap();
    assert!(list_result.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn create_collection_special_chars() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let collection_name_list = FILENAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", random_collection_name()))
        .collect::<Vec<String>>();

    for collection_name in collection_name_list {
        let create_collection_result = workspace
            .create_collection(
                &ctx,
                &CreateCollectionInput {
                    name: collection_name.clone(),
                    order: None,
                    external_path: None,
                    repo: None,
                    icon_path: None,
                },
            )
            .await;

        let create_collection_output = create_collection_result.unwrap();
        let collections = workspace.collections(&ctx).await.unwrap();

        assert!(collections.contains_key(&create_collection_output.id));

        // Verify the directory was created
        assert!(create_collection_output.abs_path.exists());

        // Verify the db entry was created
        let id = create_collection_output.id;
        let item_store = workspace.__storage().item_store();
        let entity: CollectionCacheEntity = GetItem::get(item_store.as_ref(), collection_key(id))
            .unwrap()
            .deserialize()
            .unwrap();
        assert_eq!(
            entity,
            CollectionCacheEntity {
                order: None,
                external_abs_path: None
            }
        );
    }

    cleanup().await;
}

#[tokio::test]
async fn create_collection_with_order() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: Some(42),
                external_path: None,
                repo: None,
                icon_path: None,
            },
        )
        .await;

    let create_collection_output = create_collection_result.unwrap();
    let collections = workspace.collections(&ctx).await.unwrap();

    assert_eq!(collections.len(), 1);
    // Verify the order is correctly stored
    let order = collections.iter().next().unwrap().1.read().await.order;
    assert_eq!(order, Some(42));

    // Verify the directory was created
    assert!(create_collection_output.abs_path.exists());

    // Verify the db entry was created
    let id = create_collection_output.id;
    let item_store = workspace.__storage().item_store();
    let entity: CollectionCacheEntity = GetItem::get(item_store.as_ref(), collection_key(id))
        .unwrap()
        .deserialize()
        .unwrap();
    assert_eq!(
        entity,
        CollectionCacheEntity {
            order: Some(42),
            external_abs_path: None
        }
    );

    cleanup().await;
}

#[tokio::test]
async fn create_collection_with_repo() {
    let (ctx, _workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let repo = "https://github.com/moss-foundation/sapic.git".to_string();
    let normalized_repo = "github.com/moss-foundation/sapic";
    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: None,
                external_path: None,
                repo: Some(repo),
                icon_path: None,
            },
        )
        .await;

    let create_collection_output = create_collection_result.unwrap();
    let collections = workspace.collections(&ctx).await.unwrap();

    assert_eq!(collections.len(), 1);

    // Verify the directory was created
    assert!(create_collection_output.abs_path.exists());

    // Verify that the repo is stored in the manifest model
    let collection = collections.iter().next().unwrap().1.read().await;
    assert_eq!(
        collection.manifest().await.repository,
        Some(normalized_repo.to_string())
    );

    cleanup().await;
}

#[tokio::test]
async fn create_collection_with_icon() {
    let (ctx, workspace_path, mut workspace, cleanup) = setup_test_workspace().await;

    let collection_name = random_collection_name();
    let input_icon_path = workspace_path.join("test_icon.png");
    generate_random_icon(&input_icon_path);

    let create_collection_result = workspace
        .create_collection(
            &ctx,
            &CreateCollectionInput {
                name: collection_name.clone(),
                order: None,
                external_path: None,
                repo: None,
                icon_path: Some(input_icon_path.clone()),
            },
        )
        .await;

    let create_collection_output = create_collection_result.unwrap();
    let collections = workspace.collections(&ctx).await.unwrap();

    assert_eq!(collections.len(), 1);

    let collection_path = create_collection_output.abs_path;
    // Verify the directory was created
    assert!(collection_path.exists());

    // Verify that the icon is stored in the assets folder
    assert!(
        collection_path
            .join(ASSETS_DIR)
            .join(COLLECTION_ICON_FILENAME)
            .exists()
    );
    cleanup().await;
}
