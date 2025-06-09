use moss_collection::models::{
    operations::{CreateEntryInput, DeleteEntryInput, UpdateEntryInput, UpdateEntryOutput},
    types::{Classification, PathChangeKind},
};
use moss_common::api::OperationError;
use moss_testutils::{fs_specific::FOLDERNAME_SPECIAL_CHARS, random_name::random_request_name};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use crate::shared::{
    create_test_collection, find_id_by_path, random_dir_name, request_folder_name,
};

mod shared;

#[tokio::test]
async fn update_entry_rename_request_success() {
    let (collection_path, mut collection) = create_test_collection().await;

    let old_name = random_request_name();
    let new_name = format!("{old_name}_new");

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join(&old_name),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let id = find_id_by_path(
        &create_result.virtual_changes,
        &Path::new("requests").join(&old_name),
    )
    .unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id,
            name: Some(new_name.clone()),
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
    // requests\\{new_name}.request
    // requests\\{new_name}.request\\get.sapic

    // Virtual
    // requests\\{new_name}

    let old_request_path = Path::new("requests").join(request_folder_name(&old_name));
    let new_request_path = Path::new("requests").join(request_folder_name(&new_name));

    assert_eq!(physical_changes.len(), 2);
    assert!(
        physical_changes
            .iter()
            .any(|(path, _id, kind)| path.to_path_buf() == new_request_path
                && kind == &PathChangeKind::Updated)
    );
    assert!(
        physical_changes
            .iter()
            .any(
                |(path, _id, kind)| path.to_path_buf() == new_request_path.join("get.sapic")
                    && kind == &PathChangeKind::Updated
            )
    );

    assert_eq!(virtual_changes.len(), 1);
    assert!(
        virtual_changes
            .iter()
            .any(
                |(path, _id, kind)| path.to_path_buf() == Path::new("requests").join(&new_name)
                    && kind == &PathChangeKind::Updated
            )
    );

    assert!(!collection_path.join(old_request_path).exists());
    assert!(collection_path.join(new_request_path).exists());
    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_request_no_change() {
    let (collection_path, mut collection) = create_test_collection().await;

    let request_name = random_request_name();
    let destination = Path::new("requests").join(&request_name);
    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
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
    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_request_same_name() {
    let (collection_path, mut collection) = create_test_collection().await;
    let request_name = random_request_name();
    let destination = Path::new("requests").join(&request_name);

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id,
            name: Some(request_name.clone()),
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

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_request_already_exists() {
    let (collection_path, mut collection) = create_test_collection().await;
    let first_destination = Path::new("requests").join("first");
    let second_destination = Path::new("requests").join("second");

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: first_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
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
            name: Some("second".to_string()),
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

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_request_already_exists_dir() {
    let (collection_path, mut collection) = create_test_collection().await;
    let first_destination = Path::new("requests").join("first");
    let second_destination = Path::new("requests").join("second");

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: first_destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
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
            name: Some("second".to_string()),
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

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_request_nonexistent_key() {
    let (collection_path, mut collection) = create_test_collection().await;
    let request_name = random_request_name();
    let destination = Path::new("requests").join(&request_name);
    let request_name_new = format!("{request_name}_new");

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    // Remove the entry first and try renaming
    let _ = collection
        .delete_entry(DeleteEntryInput { id })
        .await
        .unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id,
            name: Some(request_name_new.clone()),
            classification: None,
            specification: None,
            protocol: None,
            order: None,
        })
        .await;

    assert!(matches!(update_result, Err(OperationError::NotFound(..))));

    // Wait for the deletion task to complete
    tokio::time::sleep(Duration::from_millis(500)).await;
    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_request_fs_deleted() {
    let (collection_path, mut collection) = create_test_collection().await;
    let request_name = random_request_name();
    let destination = Path::new("requests").join(&request_name);
    let request_new_name = format!("{request_name}_new");

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();

    let request_path = Path::new("requests").join(request_folder_name(&request_name));

    // Delete the request from filesystem
    tokio::fs::remove_dir_all(&collection_path.join(&request_path))
        .await
        .unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id,
            name: Some(request_new_name.clone()),
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
async fn update_entry_rename_request_nested() {
    let (collection_path, mut collection) = create_test_collection().await;
    let request_name = random_request_name();
    let request_new_name = format!("{request_name}_new");
    let dir_name = random_dir_name();
    let dir_path = PathBuf::from("requests").join(&dir_name);

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: dir_path.join(&request_name),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let request_id = find_id_by_path(
        &create_result.virtual_changes,
        &dir_path.join(&request_name),
    )
    .unwrap();

    let update_result = collection
        .update_entry(UpdateEntryInput {
            id: request_id,
            name: Some(request_new_name.clone()),
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
    // requests\\{dir_name}\\{request_name_new}.request
    // requests\\{dir_name}\\{request_name_new}.request\\get.sapic

    // Virtual
    // requests\\{dir_name}\\{request_name_new}
    assert_eq!(physical_changes.len(), 2);
    assert!(
        physical_changes
            .iter()
            .any(|(path, _id, kind)| path.to_path_buf()
                == dir_path.join(&request_folder_name(&request_new_name))
                && kind == &PathChangeKind::Updated)
    );
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == dir_path
                .join(&request_folder_name(&request_new_name))
                .join("get.sapic")
            && kind == &PathChangeKind::Updated
    }));

    assert_eq!(virtual_changes.len(), 1);
    assert!(
        virtual_changes
            .iter()
            .any(
                |(path, _id, kind)| path.to_path_buf() == dir_path.join(&request_new_name)
                    && kind == &PathChangeKind::Updated
            )
    );

    let old_request_path = collection_path.join(dir_path.join(&request_folder_name(&request_name)));
    let new_request_path =
        collection_path.join(dir_path.join(&request_folder_name(&request_new_name)));
    assert!(!old_request_path.exists());
    assert!(new_request_path.exists());
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn update_entry_rename_request_special_chars() {
    let (collection_path, mut collection) = create_test_collection().await;
    for char in FOLDERNAME_SPECIAL_CHARS {
        let request_name = random_request_name();
        let new_request_name = format!("{request_name}{char}");
        let destination = Path::new("requests").join(&request_name);
        let new_destination = Path::new("requests").join(&new_request_name);
        let create_result = collection
            .create_entry(CreateEntryInput {
                destination: destination.clone(),
                classification: Classification::Request,
                specification: None,
                protocol: None,
                order: None,
                is_dir: false,
            })
            .await
            .unwrap();

        let id = find_id_by_path(&create_result.virtual_changes, &destination).unwrap();
        let update_result = collection
            .update_entry(UpdateEntryInput {
                id,
                name: Some(new_request_name.clone()),
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
        // requests\\{encoded_new_name}.request
        // requests\\{encoded_new_name}.request\\get.sapic

        // Virtual
        // requests\\{new_name}
        let old_request_path = Path::new("requests").join(request_folder_name(&request_name));
        let new_request_path = Path::new("requests").join(request_folder_name(&new_request_name));
        assert_eq!(physical_changes.len(), 2);
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == new_request_path && kind == &PathChangeKind::Updated
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == new_request_path.join("get.sapic")
                && kind == &PathChangeKind::Updated
        }));

        assert_eq!(virtual_changes.len(), 1);
        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == new_destination && kind == &PathChangeKind::Updated
        }));

        assert!(!collection_path.join(&old_request_path).exists());
        assert!(collection_path.join(&new_request_path).exists());
        tokio::fs::remove_dir_all(collection_path.join(&new_request_path))
            .await
            .unwrap();
    }
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
