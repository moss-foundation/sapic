#![cfg(feature = "integration-tests")]
use atomic_fs::Rollback;
use nanoid::nanoid;
use std::path::{Path, PathBuf};

pub async fn setup_rollback() -> (Rollback, PathBuf) {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(nanoid!(10));
    std::fs::create_dir_all(&test_path).unwrap();
    (
        Rollback::new(test_path.join("tmp")).await.unwrap(),
        test_path,
    )
}
