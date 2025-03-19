pub fn random_workspace_name() -> String {
    format!("Test_{}_Workspace", random_string(10))
}

pub fn random_collection_name() -> String {
    format!("Test_{}_Collection", random_string(10))
}

fn random_string(length: usize) -> String {
    use rand::{distr::Alphanumeric, Rng};

    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
