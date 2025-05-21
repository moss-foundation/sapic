use moss_collection::models::operations::{
    CreateEntryInput, DeleteEntryInput, UpdateEntryInput, UpdateEntryOutput,
};
use moss_collection::models::types::{Classification, PathChangeKind};
use moss_common::api::OperationError;
use moss_common::sanitized::sanitize;
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_name;
use std::path::Path;
use std::time::Duration;

use crate::shared::{find_id_by_path, random_dir_name, set_up_test_collection};
mod shared;

#[tokio::test]
async fn update_entry_rename_dir_success() {
    let (collection_path, collection) = set_up_test_collection().await;

    let dir_name = random_dir_name();
    let new_dir_name = format!("{}_new_dir", &dir_name);
    let destination = Path::new("requests").join(&dir_name);
    let new_destination = Path::new("requests").join(&new_dir_name);
    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id,
            name: Some(new_dir_name.clone()),
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    let UpdateEntryOutput {
        physical_changes,
        virtual_changes,
    } = update_result.unwrap();

    // Physical
    // requests\\{new_dir_name}
    // requests\\{new_dir_name}\\folder.sapic

    // Virtual
    // requests\\{new_dir_name}

    assert_eq!(physical_changes.len(), 2);
    assert!(
        physical_changes
            .iter()
            .any(|(path, _id, kind)| path.to_path_buf() == new_destination
                && kind == &PathChangeKind::Updated)
    );
    assert!(
        physical_changes
            .iter()
            .any(
                |(path, _id, kind)| path.to_path_buf() == new_destination.join("folder.sapic")
                    && kind == &PathChangeKind::Updated
            )
    );

    assert_eq!(virtual_changes.len(), 1);
    assert!(
        virtual_changes
            .iter()
            .any(|(path, _id, kind)| path.to_path_buf() == new_destination
                && kind == &PathChangeKind::Updated)
    );

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_dir_no_change() {
    let (collection_path, collection) = set_up_test_collection().await;
    let destination = Path::new("requests").join(&random_dir_name());

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id,
            name: None,
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    let UpdateEntryOutput {
        physical_changes,
        virtual_changes,
    } = update_result.unwrap();

    assert!(physical_changes.is_empty());
    assert!(virtual_changes.is_empty());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_dir_same_name() {
    let (collection_path, collection) = set_up_test_collection().await;
    let dir_name = random_dir_name();
    let destination = Path::new("requests").join(&dir_name);

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id,
            name: Some(dir_name.clone()),
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    let UpdateEntryOutput {
        physical_changes,
        virtual_changes,
    } = update_result.unwrap();

    assert!(physical_changes.is_empty());
    assert!(virtual_changes.is_empty());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_dir_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;
    let first_name = "first";
    let second_name = "second";
    let first_destination = Path::new("requests").join(&first_name);
    let second_destination = Path::new("requests").join(&second_name);

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: first_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let first_id = find_id_by_path(&create_result.virtual_changes, &first_destination).unwrap();

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: second_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id: first_id,
            name: Some(second_name.to_string()),
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    assert!(matches!(
        update_result,
        Err(OperationError::AlreadyExists(..))
    ));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_dir_already_exists_entry() {
    let (collection_path, collection) = set_up_test_collection().await;
    let first_name = "first";
    let second_name = "second";
    let first_destination = Path::new("requests").join(&first_name);
    let second_destination = Path::new("requests").join(&second_name);

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: first_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let first_id = find_id_by_path(&create_result.virtual_changes, &first_destination).unwrap();

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: second_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id: first_id,
            name: Some(second_name.to_string()),
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    assert!(matches!(
        update_result,
        Err(OperationError::AlreadyExists(..))
    ));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_dir_nonexistent_key() {
    let (collection_path, collection) = set_up_test_collection().await;
    let dir_name = random_dir_name();
    let new_dir_name = format!("{dir_name}_new");
    let destination = Path::new("requests").join(&dir_name);
    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    // Remove the dir entry first and try to update it
    let _ = collection
        .delete_entry(DeleteEntryInput { id })
        .await
        .unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id,
            name: Some(new_dir_name.clone()),
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    assert!(matches!(update_result, Err(OperationError::NotFound(..))));

    // Wait for the deletion task to complete
    tokio::time::sleep(Duration::from_millis(500)).await;

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_dir_fs_deleted() {
    let (collection_path, collection) = set_up_test_collection().await;
    let dir_name = random_dir_name();
    let new_dir_name = format!("{dir_name}_new");

    let destination = Path::new("requests").join(&dir_name);
    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    // Delete the dir from the filesystem
    tokio::fs::remove_dir_all(&collection_path.join(&destination))
        .await
        .unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id,
            name: Some(new_dir_name.clone()),
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    assert!(matches!(update_result, Err(OperationError::NotFound(..))));
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_dir_nested() {
    let (collection_path, collection) = set_up_test_collection().await;
    let outer_name = random_dir_name();
    let inner_name = random_request_name();
    let inner_new_name = format!("{inner_name}_new");

    let outer_destination = Path::new("requests").join(&outer_name);
    let inner_destination = outer_destination.join(&inner_name);
    let inner_new_destination = outer_destination.join(&inner_new_name);

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: outer_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let create_inner_result = collection
        .create_entry(CreateEntryInput {
            destination: inner_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let inner_id =
        find_id_by_path(&create_inner_result.virtual_changes, &inner_destination).unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id: inner_id,
            name: Some(inner_new_name.clone()),
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    let UpdateEntryOutput {
        physical_changes,
        virtual_changes,
    } = update_result.unwrap();

    // Physical
    // requests\\{outer_dir}\\{inner_dir_new}
    // requests\\{outer_dir}\\{inner_dir_new}\\folder.sapic

    // Virtual
    // requests\\{outer_dir}\\{inner_dir_new}

    assert_eq!(physical_changes.len(), 2);
    assert!(
        physical_changes
            .iter()
            .any(
                |(path, _id, kind)| path.to_path_buf() == inner_new_destination
                    && kind == &PathChangeKind::Updated
            )
    );
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == inner_new_destination.join("folder.sapic")
            && kind == &PathChangeKind::Updated
    }));

    assert_eq!(virtual_changes.len(), 1);
    assert!(
        virtual_changes.iter().any(
            |(path, _id, kind)| path.to_path_buf() == inner_new_destination
                && kind == &PathChangeKind::Updated
        )
    );

    assert!(!collection_path.join(&inner_destination).exists());
    assert!(collection_path.join(&inner_new_destination).exists());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_dir_with_content() {
    let (collection_path, collection) = set_up_test_collection().await;
    let outer_name = random_dir_name();
    let outer_new_name = format!("{outer_name}_new");
    let inner_name = random_request_name();

    let outer_destination = Path::new("requests").join(&outer_name);
    let outer_new_destination = Path::new("requests").join(&outer_new_name);
    let inner_destination = outer_destination.join(&inner_name);

    let create_outer_result = collection
        .create_entry(CreateEntryInput {
            destination: outer_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: inner_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let outer_id =
        find_id_by_path(&create_outer_result.virtual_changes, &outer_destination).unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id: outer_id,
            name: Some(outer_new_name.clone()),
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    let UpdateEntryOutput {
        physical_changes,
        virtual_changes,
    } = update_result.unwrap();

    // Physical
    // requests\\{outer_dir_new}
    // requests\\{outer_dir_new}\\folder.sapic
    // requests\\{outer_dir_new}\\{inner_dir}
    // requests\\{outer_dir_new}\\{inner_dir}\\folder.sapic

    // Virtual
    // requests\\{outer_dir_new}
    // requests\\{outer_dir_new}\\{inner_dir}

    assert_eq!(physical_changes.len(), 4);
    assert!(
        physical_changes
            .iter()
            .any(
                |(path, _id, kind)| path.to_path_buf() == outer_new_destination
                    && kind == &PathChangeKind::Updated
            )
    );
    assert!(
        physical_changes
            .iter()
            .any(|(path, _id, kind)| path.to_path_buf()
                == outer_new_destination.join("folder.sapic")
                && kind == &PathChangeKind::Updated)
    );
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == outer_new_destination.join(&inner_name)
            && kind == &PathChangeKind::Updated
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == outer_new_destination.join(&inner_name).join("folder.sapic")
            && kind == &PathChangeKind::Updated
    }));

    assert_eq!(virtual_changes.len(), 2);
    assert!(
        virtual_changes.iter().any(
            |(path, _id, kind)| path.to_path_buf() == outer_new_destination
                && kind == &PathChangeKind::Updated
        )
    );
    assert!(
        virtual_changes
            .iter()
            .any(
                |(path, _id, kind)| path.to_path_buf() == outer_new_destination.join(&inner_name)
                    && kind == &PathChangeKind::Updated
            )
    );

    assert!(!collection_path.join(&outer_destination).exists());
    assert!(collection_path.join(&outer_new_destination).exists());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_dir_special_chars() {
    let (collection_path, collection) = set_up_test_collection().await;
    for char in FOLDERNAME_SPECIAL_CHARS {
        let dir_name = random_dir_name();
        let new_dir_name = format!("{dir_name}{char}");
        let destination = Path::new("requests").join(&dir_name);
        let new_destination = Path::new("requests").join(&new_dir_name);
        let create_result = collection
            .create_entry(CreateEntryInput {
                destination: destination.clone(),
                classification: Classification::Request,
                specification: None,
                protocol: None,
                order: None,
                is_dir: true,
            })
            .await
            .unwrap();

        let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();
        let update_result = collection
            .update_entry(UpdateEntryInput {
                id,
                name: Some(new_dir_name.clone()),
                classification: None,
                specification: None,
                protocol: None,
                order: None,
            })
            .await;

        let UpdateEntryOutput {
            physical_changes,
            virtual_changes,
        } = update_result.unwrap();

        // Physical
        // requests\\{encoded_new_name}
        // requests\\{encoded_new_name}\\folder.sapic

        // Virtual
        // requests\\{encoded_new_name}
        let old_dir_path = Path::new("requests").join(&dir_name);
        let new_dir_path = Path::new("requests").join(sanitize(&new_dir_name));

        assert_eq!(physical_changes.len(), 2);
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == new_dir_path && kind == &PathChangeKind::Updated
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == new_dir_path.join("folder.sapic")
                && kind == &PathChangeKind::Updated
        }));

        assert_eq!(virtual_changes.len(), 1);
        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == new_destination && kind == &PathChangeKind::Updated
        }));

        assert!(!collection_path.join(&old_dir_path).exists());
        assert!(collection_path.join(&new_dir_path).exists());
        tokio::fs::remove_dir_all(collection_path.join(&new_dir_path))
            .await
            .unwrap();
    }
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
