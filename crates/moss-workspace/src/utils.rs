// We generate a random names to avoid situations where parallel
// tests attempt to create a folder that was already created by another parallel test.

#[cfg(test)]
pub(crate) fn random_workspace_name() -> String {
    format!("Test_{}_Workspace", random_string(10))
}

#[cfg(test)]
pub(crate) fn random_collection_name() -> String {
    format!("Test_{}_Collection", random_string(10))
}

#[cfg(test)]
fn random_string(length: usize) -> String {
    use rand::{distr::Alphanumeric, Rng};

    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
