mod shared;

use atomic_fs::{RemoveOptions, remove_dir};

use crate::shared::setup_rollback;

#[tokio::test]
pub async fn test_remove_dir_success() {
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

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_remove_dir_with_content() {
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

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_remove_dir_ignore_when_not_exist() {
    let (mut rb, test_path) = setup_rollback().await;

    // Removing non-existent directory
    let target = test_path.join("1");

    assert!(
        remove_dir(
            &mut rb,
            &target,
            RemoveOptions {
                ignore_if_not_exists: true
            }
        )
        .await
        .is_ok()
    );

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_remove_dir_not_ignore_when_not_exist() {
    let (mut rb, test_path) = setup_rollback().await;

    // Removing non-existent directory
    let target = test_path.join("1");
    assert!(
        remove_dir(
            &mut rb,
            &target,
            RemoveOptions {
                ignore_if_not_exists: false,
            }
        )
        .await
        .is_err()
    );

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}
