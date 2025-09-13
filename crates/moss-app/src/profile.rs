use moss_user::models::primitives::{AccountId, AccountKind, ProfileId, SessionKind};
use serde::{Deserialize, Serialize};

pub(crate) const PROFILES_REGISTRY_FILE: &str = "profiles.json";
pub(crate) const PROFILE_SETTINGS_FILE: &str = "settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ProfileRegistryItem {
    pub id: ProfileId,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,

    pub accounts: Vec<ProfileRegistryAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ProfileRegistryAccount {
    pub id: AccountId,
    pub username: String,
    pub host: String,
    pub kind: AccountKind,
    pub metadata: ProfileRegistryAccountMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ProfileRegistryAccountMetadata {
    pub session_kind: SessionKind,
}
