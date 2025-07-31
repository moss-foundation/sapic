#![cfg(feature = "integration-tests")]
use atomic_fs::create_dir;

use crate::shared::setup_rollback;
mod shared;

#[tokio::test]
pub async fn test_create_dir_success() {
    let (mut rb, test_path) = setup_rollback();
    let target = test_path.join("1");

    create_dir(&mut rb, &target).await.unwrap();

    assert!(target.exists());
    assert!(target.is_dir());
    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}
#[tokio::test]
pub async fn test_create_dir_missing_parent() {
    let (mut rb, test_path) = setup_rollback();

    // Missing parent folder
    assert!(
        create_dir(&mut rb, test_path.join("missing").join("1"))
            .await
            .is_err()
    );

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}
