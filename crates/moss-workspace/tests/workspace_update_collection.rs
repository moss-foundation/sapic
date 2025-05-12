//
// TODO: Implement when we have a way to rename collections
//

// mod shared;

// use moss_common::api::OperationError;
// use moss_fs::utils::encode_name;
// use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_collection_name};
// use moss_workspace::models::operations::{CreateCollectionInput, UpdateCollectionEntryInput};
// use moss_workspace::models::types::CollectionInfo;
// use moss_workspace::workspace::COLLECTIONS_DIR;

// use crate::shared::setup_test_workspace;

// #[tokio::test]
// async fn rename_collection_success() {
//     let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

//     let old_collection_name = random_collection_name();
//     let create_collection_output = workspace
//         .create_collection(CreateCollectionInput {
//             name: old_collection_name.clone(),
//         })
//         .await
//         .unwrap();

//     let new_collection_name = random_collection_name();
//     let result = workspace
//         .update_collection_entry(UpdateCollectionEntryInput {
//             id: create_collection_output.id,
//             new_name: Some(new_collection_name.clone()),
//         })
//         .await
//         .unwrap();

//     // assert!(result.is_ok());

//     // let rename_collection_output = result.unwrap();
//     // assert!(rename_collection_output.abs_path.exists());
//     // assert!(!create_collection_output.abs_path.exists());

//     // // Check updating collections
//     // let describe_output = workspace.describe().await.unwrap();
//     // assert_eq!(describe_output.collections.len(), 1);
//     // assert_eq!(
//     //     describe_output.collections[0],
//     //     CollectionInfo {
//     //         id: create_collection_output.id,
//     //         display_name: new_collection_name,
//     //         order: None,
//     //     }
//     // );

//     // cleanup().await;
// }

// #[tokio::test]
// async fn rename_collection_empty_name() {
//     let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

//     let old_collection_name = random_collection_name();
//     let create_collection_output = workspace
//         .create_collection(CreateCollectionInput {
//             name: old_collection_name.clone(),
//         })
//         .await
//         .unwrap();

//     let new_collection_name = "".to_string();
//     let rename_collection_result = workspace
//         .update_collection_entry(UpdateCollectionEntryInput {
//             id: create_collection_output.id,
//             new_name: Some(new_collection_name.clone()),
//         })
//         .await;

//     assert!(matches!(
//         rename_collection_result,
//         Err(OperationError::Validation(_))
//     ));

//     cleanup().await;
// }

// #[tokio::test]
// async fn rename_collection_unchanged() {
//     let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

//     let old_collection_name = random_collection_name();
//     let create_collection_output = workspace
//         .create_collection(CreateCollectionInput {
//             name: old_collection_name.clone(),
//         })
//         .await
//         .unwrap();

//     let new_collection_name = old_collection_name;
//     let rename_collection_result = workspace
//         .update_collection_entry(UpdateCollectionEntryInput {
//             id: create_collection_output.id,
//             new_name: Some(new_collection_name),
//         })
//         .await;

//     assert!(rename_collection_result.is_ok());

//     cleanup().await;
// }

// #[tokio::test]
// async fn rename_collection_already_exists() {
//     let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

//     let existing_collection_name = random_collection_name();

//     // Create an existing collection
//     workspace
//         .create_collection(CreateCollectionInput {
//             name: existing_collection_name.clone(),
//         })
//         .await
//         .unwrap();

//     let new_collection_name = random_collection_name();
//     // Create a collection to test renaming
//     let create_collection_output = workspace
//         .create_collection(CreateCollectionInput {
//             name: new_collection_name.clone(),
//         })
//         .await
//         .unwrap();

//     // Try renaming the new collection to an existing collection name
//     let result = workspace
//         .update_collection_entry(UpdateCollectionEntryInput {
//             id: create_collection_output.id,
//             new_name: Some(existing_collection_name.clone()),
//         })
//         .await;
//     assert!(matches!(result, Err(OperationError::AlreadyExists { .. })));

//     // Clean up
//     cleanup().await;
// }

// #[tokio::test]
// async fn rename_collection_special_chars() {
//     let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

//     let collection_name = random_collection_name();
//     let create_collection_output = workspace
//         .create_collection(CreateCollectionInput {
//             name: collection_name.clone(),
//         })
//         .await
//         .unwrap();

//     for char in FILENAME_SPECIAL_CHARS {
//         let new_collection_name = format!("{collection_name}{char}");
//         let expected_path = _workspace_path
//             .join(COLLECTIONS_DIR)
//             .join(encode_name(&new_collection_name));

//         let rename_collection_result = workspace
//             .update_collection_entry(UpdateCollectionEntryInput {
//                 id: create_collection_output.id,
//                 new_name: Some(new_collection_name.clone()),
//             })
//             .await;
//         assert!(rename_collection_result.is_ok());
//         assert!(expected_path.exists());

//         // Checking updating collections
//         let describe_output = workspace.describe().await.unwrap();
//         assert_eq!(describe_output.collections.len(), 1);
//         assert_eq!(
//             describe_output.collections[0],
//             CollectionInfo {
//                 id: create_collection_output.id,
//                 display_name: new_collection_name.clone(),
//                 order: None,
//             }
//         );
//     }

//     cleanup().await;
// }

// #[tokio::test]
// async fn rename_collection_nonexistent_id() {
//     let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

//     // Use a random ID that doesn't exist
//     let nonexistent_id = moss_common::models::primitives::Identifier::new(&std::sync::Arc::new(
//         std::sync::atomic::AtomicUsize::new(100),
//     ));

//     let result = workspace
//         .update_collection_entry(UpdateCollectionEntryInput {
//             id: nonexistent_id,
//             new_name: Some(random_collection_name()),
//         })
//         .await;

//     assert!(matches!(result, Err(OperationError::NotFound { .. })));

//     cleanup().await;
// }
