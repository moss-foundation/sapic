mod shared;

use atomic_fs::{CreateOptions, create_file_with};

use crate::shared::setup_rollback;

#[tokio::test]
pub async fn test_create_file_with_success() {
    let (mut rb, test_path) = setup_rollback().await;

    let data = "Hello World".as_bytes();
    let target = test_path.join("file.txt");

    create_file_with(
        &mut rb,
        &target,
        CreateOptions {
            overwrite: false,
            ignore_if_exists: false,
        },
        data,
    )
    .await
    .unwrap();

    assert!(target.is_file());
    let content = tokio::fs::read(&target).await.unwrap();
    assert_eq!(content, data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_create_file_with_ignore_if_exists() {
    let (mut rb, test_path) = setup_rollback().await;

    let old_data = "Hello World".as_bytes();
    let new_data = "New".as_bytes();
    let target = test_path.join("file.txt");
    tokio::fs::write(&target, old_data).await.unwrap();

    // Skipping when a file already exists
    create_file_with(
        &mut rb,
        &target,
        CreateOptions {
            overwrite: false,
            ignore_if_exists: true,
        },
        new_data,
    )
    .await
    .unwrap();

    assert!(tokio::fs::read(&target).await.unwrap() == old_data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_create_file_with_overwrite_existing_file() {
    let (mut rb, test_path) = setup_rollback().await;
    let old_data = "Hello World".as_bytes();
    let new_data = "42".as_bytes();
    let target = test_path.join("file.txt");

    tokio::fs::write(&target, old_data).await.unwrap();

    create_file_with(
        &mut rb,
        &target,
        CreateOptions {
            overwrite: true,
            ignore_if_exists: false,
        },
        new_data,
    )
    .await
    .unwrap();

    let content = tokio::fs::read(&target).await.unwrap();
    assert_eq!(content, new_data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_create_file_with_append_to_existing_file() {
    let (mut rb, test_path) = setup_rollback().await;
    let old_data = "Hello World".as_bytes();
    let new_data = "42".as_bytes();
    let target = test_path.join("file.txt");

    tokio::fs::write(&target, old_data).await.unwrap();

    create_file_with(
        &mut rb,
        &target,
        CreateOptions {
            overwrite: false,
            ignore_if_exists: false,
        },
        new_data,
    )
    .await
    .unwrap();

    let content = tokio::fs::read(&target).await.unwrap();
    let complete_data = old_data
        .into_iter()
        .chain(new_data.into_iter())
        .cloned()
        .collect::<Vec<_>>();

    assert_eq!(content, complete_data);

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}
