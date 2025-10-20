use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::primitives::{AccountId, AccountKind, ProfileId, SessionKind};

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ProfileInfo {
    pub id: ProfileId,
    pub name: String,
    pub accounts: Vec<AccountInfo>,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AccountInfo {
    pub id: AccountId,
    pub username: String,
    pub host: String,
    pub kind: AccountKind,
    pub method: SessionKind,
    // pub metadata: AccountMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AccountMetadata {
    pub expires_at: Option<DateTime<Utc>>,
}
