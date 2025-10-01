mod shared;

use atomic_fs::{CreateOptions, create_file};

use crate::shared::setup_rollback;

#[tokio::test]
pub async fn test_create_file_success() {
    let (mut rb, test_path) = setup_rollback().await;

    let target = test_path.join("file.txt");

    create_file(
        &mut rb,
        &target,
        CreateOptions {
            overwrite: false,
            ignore_if_exists: false,
        },
    )
    .await
    .unwrap();

    assert!(target.is_file());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_create_file_ignore_if_exists() {
    let (mut rb, test_path) = setup_rollback().await;

    let target = test_path.join("file.txt");
    let content = "test".as_bytes();

    tokio::fs::write(&target, content).await.unwrap();
    create_file(
        &mut rb,
        &target,
        CreateOptions {
            overwrite: false,
            ignore_if_exists: true,
        },
    )
    .await
    .unwrap();

    assert!(target.is_file());
    assert!(tokio::fs::read(&target).await.unwrap() == content);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_create_file_overwrite_truncate_existing_file() {
    let (mut rb, test_path) = setup_rollback().await;

    let data = "Hello World".as_bytes();
    let target = test_path.join("file.txt");

    tokio::fs::write(&target, data).await.unwrap();

    // create_file with overwrite will truncate existing content
    create_file(
        &mut rb,
        &target,
        CreateOptions {
            overwrite: true,
            ignore_if_exists: false,
        },
    )
    .await
    .unwrap();

    assert!(target.is_file());
    let new_data = tokio::fs::read(&target).await.unwrap();
    assert!(new_data.is_empty());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_create_file_non_overwrite_preserve_existing_file() {
    let (mut rb, test_path) = setup_rollback().await;

    let data = "Hello World".as_bytes();
    let target = test_path.join("file.txt");

    tokio::fs::write(&target, data).await.unwrap();

    // create_file without overwrite will preserve existing content
    create_file(
        &mut rb,
        &target,
        CreateOptions {
            overwrite: false,
            ignore_if_exists: false,
        },
    )
    .await
    .unwrap();

    assert!(target.is_file());
    let new_data = tokio::fs::read(&target).await.unwrap();
    assert_eq!(new_data, data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}
