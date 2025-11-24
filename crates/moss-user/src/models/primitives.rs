use moss_id_macro::ids;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[cfg(any(test, feature = "integration-tests"))]
use sapic_core::context::ContextValue;

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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum SessionKind {
    OAuth,
    PAT,
}

impl From<SessionKind> for AccountAuthMethodKindForFrontend {
    fn from(value: SessionKind) -> Self {
        match value {
            SessionKind::OAuth => AccountAuthMethodKindForFrontend::OAuth,
            SessionKind::PAT => AccountAuthMethodKindForFrontend::PAT,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, TS)]
#[serde(rename = "AccountAuthMethodKind")]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum AccountAuthMethodKindForFrontend {
    OAuth,
    PAT,
}
