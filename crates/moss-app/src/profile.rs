use moss_user::models::primitives::{AccountId, AccountKind, SessionKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProfileFile {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    pub accounts: Vec<ProfileFileAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProfileFileAccount {
    pub id: AccountId,
    pub username: String,
    pub host: String,
    pub kind: AccountKind,
    pub metadata: ProfileFileAccountMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProfileFileAccountMetadata {
    pub session_kind: SessionKind,
}
