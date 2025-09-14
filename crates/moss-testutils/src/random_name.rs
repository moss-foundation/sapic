use rand::{Rng, distr::Alphanumeric};

pub fn random_workspace_name() -> String {
    format!("Test_{}_Workspace", random_string(10))
}

pub fn random_project_name() -> String {
    format!("Test_{}_Project", random_string(10))
}

pub fn random_environment_name() -> String {
    format!("Test_{}_Environment", random_string(10))
}

pub fn random_request_name() -> String {
    format!("Test_{}_Request", random_string(10))
}

pub fn random_string(length: usize) -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
