use moss_id_macro::ids;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[cfg(any(test, feature = "integration-tests"))]
use moss_applib::context::ContextValue;

ids!([ProfileId, AccountId]);

#[cfg(any(test, feature = "integration-tests"))]
impl ContextValue for AccountId {}

/// @category Primitive
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum AccountKind {
    GitHub,
    GitLab,
}

/// @category Primitive
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum SessionKind {
    OAuth,
    PAT,
}
