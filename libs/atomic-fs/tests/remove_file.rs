mod shared;

use atomic_fs::{RemoveOptions, remove_file};

use crate::shared::setup_rollback;

#[tokio::test]
pub async fn test_remove_file_success() {
    let (mut rb, test_path) = setup_rollback().await;

    let target = test_path.join("file.txt");
    tokio::fs::File::create(&target).await.unwrap();

    remove_file(
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
pub async fn test_remove_file_ignore_when_not_exist() {
    let (mut rb, test_path) = setup_rollback().await;

    let target = test_path.join("file.txt");

    assert!(
        remove_file(
            &mut rb,
            &target,
            RemoveOptions {
                ignore_if_not_exists: true,
            }
        )
        .await
        .is_ok()
    );

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}

#[tokio::test]
pub async fn test_remove_file_not_ignore_when_not_exist() {
    let (mut rb, test_path) = setup_rollback().await;

    let target = test_path.join("file.txt");

    assert!(
        remove_file(
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
