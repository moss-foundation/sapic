use atomic_fs::Rollback;
use nanoid::nanoid;
use std::path::{Path, PathBuf};

pub async fn setup_rollback() -> (Rollback, PathBuf) {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(nanoid!(10));

    let temp_path = test_path.join("tmp");
    std::fs::create_dir_all(&test_path).unwrap();
    std::fs::create_dir_all(&temp_path).unwrap();
    (Rollback::new(&temp_path).await, test_path)
}
