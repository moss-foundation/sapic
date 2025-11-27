use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::primitives::{
    AccountAuthMethodKindForFrontend, AccountId, AccountKind, ProfileId,
};

// /// @category Type
// #[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "types.ts")]
// pub struct ProfileInfo {
//     pub id: ProfileId,
//     pub name: String,
//     pub accounts: Vec<AccountInfo>,
// }

// /// @category Type
// #[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "types.ts")]
// pub struct AccountInfo {
//     pub id: AccountId,
//     pub username: String,
//     pub host: String,
//     pub kind: AccountKind,
//     #[ts(type = "AccountAuthMethodKind")]
//     pub method: AccountAuthMethodKindForFrontend,
//     pub metadata: AccountMetadata,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(optional_fields)]
// #[ts(export, export_to = "types.ts")]
// pub struct AccountMetadata {
//     pub pat_expires_at: Option<DateTime<Utc>>,
// }
