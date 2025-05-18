use crate::worktree::virtual_snapshot::Classification;

pub fn file_name_from_protocol(protocol: &str) -> String {
    format!("{}.sapic", protocol)
}

pub fn dir_name_from_classification(base_name: &str, classification: &Classification) -> String {
    format!("{}.{}", base_name, classification.as_str())
}
