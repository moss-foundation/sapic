use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::user::types::{
    AccountInfo,
    primitives::{AccountId, AccountKind, SessionKind},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsManifestItemMetadata {
    pub session_kind: SessionKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsManifestItem {
    pub id: AccountId,
    pub username: String,
    pub host: String,
    pub kind: AccountKind,
    pub metadata: AccountsManifestItemMetadata,
}

impl From<AccountInfo> for AccountsManifestItem {
    fn from(account: AccountInfo) -> Self {
        Self {
            id: account.id,
            username: account.username,
            host: account.host,
            kind: account.kind,
            metadata: AccountsManifestItemMetadata {
                session_kind: account.method.into(),
                expires_at: account.metadata.pat_expires_at,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserAccountsManifest(pub Vec<AccountsManifestItem>);
