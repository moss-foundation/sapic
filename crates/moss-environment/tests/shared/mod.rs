use std::path::PathBuf;

pub fn random_string(length: usize) -> String {
    use rand::{distr::Alphanumeric, Rng};

    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn test_environment_data() -> (PathBuf) {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data");
    let environment_name = format!("Test_{}_Environment.json", random_string(10));
    let environment_file_path = base_path.join(environment_name.clone());

    environment_file_path
}
