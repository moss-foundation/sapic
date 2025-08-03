#![cfg(feature = "integration-tests")]

use atomic_fs::{RenameOptions, rename};

use crate::shared::setup_rollback;

mod shared;

#[tokio::test]
pub async fn test_rename_success() {
    let (mut rb, test_path) = setup_rollback();

    let data = "Hello World".as_bytes();
    let source = test_path.join("old.txt");
    let dest = test_path.join("new.txt");

    tokio::fs::write(&source, data).await.unwrap();

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

    assert!(!source.exists());
    assert!(dest.exists());
    let content = tokio::fs::read(&dest).await.unwrap();
    assert_eq!(content, data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rename_different_types() {
    let (mut rb, test_path) = setup_rollback();

    let source = test_path.join("old.txt");
    let dest = test_path.join("new");

    tokio::fs::File::create(&source).await.unwrap();
    tokio::fs::create_dir(&dest).await.unwrap();

    assert!(
        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: true,
                ignore_if_exists: false
            }
        )
        .await
        .is_err()
    );

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rename_ignore_when_exists() {
    let (mut rb, test_path) = setup_rollback();

    let source = test_path.join("old.txt");
    let dest = test_path.join("new.txt");

    tokio::fs::File::create(&source).await.unwrap();
    tokio::fs::File::create(&dest).await.unwrap();

    // Since the destination already exists, this will be a no op

    rename(
        &mut rb,
        &source,
        &dest,
        RenameOptions {
            overwrite: false,
            ignore_if_exists: true,
        },
    )
    .await
    .unwrap();

    assert!(source.exists());
    assert!(dest.exists());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rename_overwrite_existing_file() {
    let (mut rb, test_path) = setup_rollback();

    let source = test_path.join("old.txt");
    let source_content = "Source".as_bytes();
    let dest = test_path.join("new.txt");
    let dest_content = "Destination".as_bytes();

    tokio::fs::write(&source, source_content).await.unwrap();
    tokio::fs::write(&dest, dest_content).await.unwrap();

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

    assert!(!source.exists());
    assert!(dest.exists());
    let content = tokio::fs::read(&dest).await.unwrap();
    assert_eq!(content, source_content);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rename_non_overwrite_already_exists() {
    let (mut rb, test_path) = setup_rollback();

    let source = test_path.join("old.txt");
    let dest = test_path.join("new.txt");

    tokio::fs::File::create(&source).await.unwrap();
    tokio::fs::create_dir(&dest).await.unwrap();

    assert!(
        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: false,
                ignore_if_exists: false,
            }
        )
        .await
        .is_err()
    );

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_rename_dir_nonempty_destination() {
    let (mut rb, test_path) = setup_rollback();

    let source = test_path.join("dir");
    let dest = test_path.join("new_dir");
    let file = dest.join("file.txt");
    tokio::fs::create_dir(&dest).await.unwrap();
    tokio::fs::write(&file, "Hello World".as_bytes())
        .await
        .unwrap();

    assert!(
        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: true,
                ignore_if_exists: false
            }
        )
        .await
        .is_err()
    );

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}
