use moss_id_macro::ids;
use sapic_core::context::ContextValue;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ids!([ProfileId, AccountId]);

impl ContextValue for ProfileId {}
impl ContextValue for AccountId {}

/// @category Primitive
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "user/primitives.ts")]
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

impl From<AccountAuthMethodKindForFrontend> for SessionKind {
    fn from(value: AccountAuthMethodKindForFrontend) -> Self {
        match value {
            AccountAuthMethodKindForFrontend::OAuth => SessionKind::OAuth,
            AccountAuthMethodKindForFrontend::PAT => SessionKind::PAT,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, TS)]
#[serde(rename = "AccountAuthMethodKind")]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "user/primitives.ts")]
pub enum AccountAuthMethodKindForFrontend {
    OAuth,
    PAT,
}
