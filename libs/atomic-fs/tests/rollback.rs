#![cfg(feature = "integration-tests")]

use atomic_fs::{
    CreateOptions, RemoveOptions, RenameOptions, create_dir, create_dir_all, create_file,
    create_file_with, remove_dir, remove_file, rename,
};

use crate::shared::setup_rollback;

mod shared;

/// -------------------------------------------
///          Simple Rollback Scenarios
/// -------------------------------------------

#[tokio::test]
pub async fn test_rollback_create_dir() {
    let (mut rb, test_path) = setup_rollback().await;

    let target = test_path.join("1");

    create_dir(&mut rb, &target).await.unwrap();
    assert!(target.is_dir());

    rb.rollback().await.unwrap();
    assert!(!target.exists());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_create_dir_all() {
    let (mut rb, test_path) = setup_rollback().await;

    let outer = test_path.join("1");
    let inner = outer.join("2");

    create_dir_all(&mut rb, &inner).await.unwrap();
    assert!(outer.is_dir());
    assert!(inner.is_dir());

    rb.rollback().await.unwrap();

    assert!(!outer.exists());
    assert!(!inner.exists());
    // Should remove only directories created during the operation
    assert!(test_path.exists());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_remove_dir() {
    let (mut rb, test_path) = setup_rollback().await;

    let target = test_path.join("1");
    tokio::fs::create_dir(&target).await.unwrap();

    remove_dir(
        &mut rb,
        &target,
        RemoveOptions {
            ignore_if_not_exists: false,
        },
    )
    .await
    .unwrap();
    assert!(!target.exists());

    rb.rollback().await.unwrap();

    // Should restore deleted directory
    assert!(target.exists());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_remove_dir_with_content() {
    let (mut rb, test_path) = setup_rollback().await;

    let target = test_path.join("1");
    let file = target.join("file.txt");
    tokio::fs::create_dir(&target).await.unwrap();
    tokio::fs::File::create(&file).await.unwrap();

    remove_dir(
        &mut rb,
        &target,
        RemoveOptions {
            ignore_if_not_exists: false,
        },
    )
    .await
    .unwrap();
    assert!(!target.exists());

    rb.rollback().await.unwrap();

    // Should restore deleted directory and its content
    assert!(target.exists());
    assert!(file.exists());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_create_file() {
    let (mut rb, test_path) = setup_rollback().await;

    let file = test_path.join("file.txt");

    create_file(
        &mut rb,
        &file,
        CreateOptions {
            overwrite: false,
            ignore_if_exists: true,
        },
    )
    .await
    .unwrap();
    assert!(file.is_file());

    rb.rollback().await.unwrap();

    assert!(!file.exists());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_create_file_truncate() {
    let (mut rb, test_path) = setup_rollback().await;

    let file = test_path.join("file.txt");

    let data = "Hello World!".as_bytes();
    // Create a test file with content
    tokio::fs::write(&file, data).await.unwrap();

    // Create a file with overwrite option will truncate it
    create_file(
        &mut rb,
        &file,
        CreateOptions {
            overwrite: true,
            ignore_if_exists: false,
        },
    )
    .await
    .unwrap();
    let data_after_truncation = tokio::fs::read(&file).await.unwrap();
    assert!(data_after_truncation.is_empty());

    rb.rollback().await.unwrap();
    // Should restore the original content before truncation
    let data_after_rollback = tokio::fs::read(&file).await.unwrap();
    assert_eq!(data_after_rollback, data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_create_file_with_content() {
    let (mut rb, test_path) = setup_rollback().await;

    let file = test_path.join("file.txt");

    let data = "Hello World!".as_bytes();

    create_file_with(
        &mut rb,
        &file,
        CreateOptions {
            overwrite: true,
            ignore_if_exists: true,
        },
        data,
    )
    .await
    .unwrap();

    rb.rollback().await.unwrap();
    // Should remove the newly created file
    assert!(!file.exists());
    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_create_file_with_overwrite_existing() {
    let (mut rb, test_path) = setup_rollback().await;

    let file = test_path.join("file.txt");

    let old_data = "Old".as_bytes();
    let new_data = "New".as_bytes();

    // Create the file with old data first
    tokio::fs::write(&file, old_data).await.unwrap();
    // Overwrite the file with new data
    create_file_with(
        &mut rb,
        &file,
        CreateOptions {
            overwrite: true,
            ignore_if_exists: false,
        },
        new_data,
    )
    .await
    .unwrap();

    rb.rollback().await.unwrap();

    // Should restore the old content of this file
    assert!(file.exists());
    let restored_data = tokio::fs::read(&file).await.unwrap();

    assert_eq!(old_data, restored_data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_create_file_with_append_to_existing() {
    let (mut rb, test_path) = setup_rollback().await;

    let file = test_path.join("file.txt");
    let old_data = "Old".as_bytes();
    let new_data = "New".as_bytes();
    tokio::fs::write(&file, old_data).await.unwrap();

    // Create the file with old data first
    tokio::fs::write(&file, old_data).await.unwrap();
    // Append the new data at the end of old data
    create_file_with(
        &mut rb,
        &file,
        CreateOptions {
            overwrite: false,
            ignore_if_exists: false,
        },
        new_data,
    )
    .await
    .unwrap();

    rb.rollback().await.unwrap();
    // Should restore the old content of this file
    assert!(file.exists());
    let restored_data = tokio::fs::read(&file).await.unwrap();

    assert_eq!(old_data, restored_data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_remove_file() {
    let (mut rb, test_path) = setup_rollback().await;
    let file = test_path.join("file.txt");
    let content = "Hello World!".as_bytes();

    tokio::fs::write(&file, content).await.unwrap();

    remove_file(
        &mut rb,
        &file,
        RemoveOptions {
            ignore_if_not_exists: false,
        },
    )
    .await
    .unwrap();

    rb.rollback().await.unwrap();
    // Should restore the file with the original content
    assert!(file.exists());
    let restored_data = tokio::fs::read(&file).await.unwrap();
    assert_eq!(content, restored_data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_rename_file() {
    let (mut rb, test_path) = setup_rollback().await;
    let source = test_path.join("source.txt");
    let dest = test_path.join("dest.txt");
    let content = "Hello World!".as_bytes();

    tokio::fs::write(&source, content).await.unwrap();

    rename(
        &mut rb,
        &source,
        &dest,
        RenameOptions {
            overwrite: false,
            ignore_if_exists: false,
        },
    )
    .await
    .unwrap();

    rb.rollback().await.unwrap();
    assert!(source.exists());
    assert!(!dest.exists());
    let restored_data = tokio::fs::read(&source).await.unwrap();
    assert_eq!(content, restored_data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_rename_file_overwrite() {
    let (mut rb, test_path) = setup_rollback().await;
    let source = test_path.join("source.txt");
    let source_data = "Source".as_bytes();
    let dest = test_path.join("dest.txt");
    let dest_data = "Dest".as_bytes();

    tokio::fs::write(&source, source_data).await.unwrap();
    tokio::fs::write(&dest, dest_data).await.unwrap();

    rename(
        &mut rb,
        &source,
        &dest,
        RenameOptions {
            overwrite: true,
            ignore_if_exists: false,
        },
    )
    .await
    .unwrap();

    rb.rollback().await.unwrap();
    // Both source and dest should be restored to the previous state
    assert!(source.exists());
    let restored_source_data = tokio::fs::read(&source).await.unwrap();
    assert_eq!(source_data, restored_source_data);

    assert!(dest.exists());
    let restored_dest_data = tokio::fs::read(&dest).await.unwrap();
    assert_eq!(dest_data, restored_dest_data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_rename_dir_with_content() {
    let (mut rb, test_path) = setup_rollback().await;
    let source = test_path.join("dir");
    let file = source.join("file.txt");
    let file_content = "Hello World!".as_bytes();
    let dest = test_path.join("new_dir");

    tokio::fs::create_dir(&source).await.unwrap();
    tokio::fs::write(&file, file_content).await.unwrap();

    rename(
        &mut rb,
        &source,
        &dest,
        RenameOptions {
            overwrite: false,
            ignore_if_exists: false,
        },
    )
    .await
    .unwrap();

    rb.rollback().await.unwrap();
    assert!(source.exists());
    assert!(file.exists());
    let restored_file_content = tokio::fs::read(&file).await.unwrap();
    assert_eq!(file_content, restored_file_content);
    assert!(!dest.exists());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rollback_rename_dir_empty_dest_exists() {
    let (mut rb, test_path) = setup_rollback().await;
    let source = test_path.join("dir");
    let file = source.join("file.txt");
    let file_content = "Hello World!".as_bytes();
    let dest = test_path.join("new_dir");

    tokio::fs::create_dir(&source).await.unwrap();
    tokio::fs::write(&file, file_content).await.unwrap();
    tokio::fs::create_dir(&dest).await.unwrap();

    rename(
        &mut rb,
        &source,
        &dest,
        RenameOptions {
            overwrite: true,
            ignore_if_exists: false,
        },
    )
    .await
    .unwrap();

    rb.rollback().await.unwrap();
    assert!(source.exists());
    assert!(file.exists());
    let restored_file_content = tokio::fs::read(&file).await.unwrap();
    assert_eq!(file_content, restored_file_content);

    // The initial empty destination directory should be restored
    assert!(dest.exists());
    let mut entries = tokio::fs::read_dir(&dest).await.unwrap();
    assert!(entries.next_entry().await.unwrap().is_none());

    tokio::fs::remove_dir_all(&dest).await.unwrap();
}

/// -------------------------------------------
///          Complex Rollback Scenarios
/// -------------------------------------------

#[tokio::test]
pub async fn test_rollback_complex() {
    // Base state:
    // folder
    // folder/inner.txt ("inner")
    // outer.txt ("outer")

    let (mut rb, test_path) = setup_rollback().await;
    let folder = test_path.join("folder");
    let inner_file = folder.join("inner.txt");
    let inner_content = "inner".as_bytes();
    let outer_file = test_path.join("outer.txt");
    let outer_content = "outer".as_bytes();

    tokio::fs::create_dir(&folder).await.unwrap();
    tokio::fs::write(&inner_file, inner_content).await.unwrap();
    tokio::fs::write(&outer_file, outer_content).await.unwrap();

    rename(
        &mut rb,
        &inner_file,
        &outer_file,
        RenameOptions {
            overwrite: true,
            ignore_if_exists: false,
        },
    )
    .await
    .unwrap();

    let inner_new_content = "new_content".as_bytes();
    create_file_with(
        &mut rb,
        &inner_file,
        CreateOptions {
            overwrite: false,
            ignore_if_exists: true,
        },
        inner_new_content,
    )
    .await
    .unwrap();

    rb.rollback().await.unwrap();
    assert!(folder.exists());
    assert!(inner_file.exists());
    assert!(outer_file.exists());
    let restored_inner_content = tokio::fs::read(&inner_file).await.unwrap();
    assert_eq!(inner_content, restored_inner_content);
    let restored_outer_content = tokio::fs::read(&outer_file).await.unwrap();
    assert_eq!(outer_content, restored_outer_content);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}
