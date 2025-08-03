#![cfg(feature = "integration-tests")]
use atomic_fs::create_dir_all;

use crate::shared::setup_rollback;

mod shared;

#[tokio::test]
pub async fn test_create_dir_all_success() {
    let (mut rb, test_path) = setup_rollback();

    let outer = test_path.join("1");
    let inner = outer.join("2");

    create_dir_all(&mut rb, &inner).await.unwrap();

    assert!(outer.is_dir());
    assert!(inner.is_dir());

    tokio::fs::remove_dir_all(&test_path).await.unwrap();
}
