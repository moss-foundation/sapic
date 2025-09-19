use serde::Deserialize;

use crate::models::primitives::LocaleId;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LocaleRegistryItem {
    pub identifier: LocaleId,
    pub display_name: String,
    pub code: String,
    pub direction: Option<String>,
    pub order: Option<isize>,
    pub is_default: Option<bool>,
}
