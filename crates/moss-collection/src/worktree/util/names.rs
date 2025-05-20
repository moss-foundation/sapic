use crate::models::types::Classification;

pub fn dir_name_from_classification(base_name: &str, classification: &Classification) -> String {
    format!("{}.{}", base_name, classification.as_str())
}
