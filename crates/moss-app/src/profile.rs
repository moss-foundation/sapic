use moss_user::models::primitives::{AccountId, AccountKind, SessionKind};
use serde::{Deserialize, Serialize};

use crate::models::primitives::{LocaleId, ThemeId};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProfileFile {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,

    pub accounts: Vec<ProfileFileAccount>,

    pub settings: ProfileSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProfileSettings {
    /// The preferred color theme for this profile.
    /// If not set, the default theme will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<ThemeId>,

    /// The preferred locale for this profile.
    /// If not set, the default locale will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<LocaleId>,

    /// The preferred zoom level for this profile.
    /// If not set, the default zoom level will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom_level: Option<f32>,
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
