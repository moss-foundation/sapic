use crate::models::primitives::LocaleId;
use serde::{Deserialize, Serialize};

pub(crate) const LOCALES_REGISTRY_FILE: &str = "locales.json";

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LocaleRegistryItem {
    pub identifier: LocaleId,
    pub display_name: String,
    pub code: String,
    pub direction: Option<String>,
    pub order: Option<isize>,
    pub is_default: Option<bool>,
    pub namespaces: Vec<String>,
}
