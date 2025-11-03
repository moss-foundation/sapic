pub mod events;
pub mod operations;
pub mod primitives;

use moss_user::models::primitives::{AccountId, AccountKind};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use ts_rs::TS;

use crate::types::primitives::*;

//
// Workspace
//

/// @category Type
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct WorkspaceInfo {
    pub id: WorkspaceId,
    pub name: String,
    pub last_opened_at: Option<i64>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Arc<Path>,
}

//
// Profile
//

/// @category Type
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct AddAccountParams {
    pub host: String,
    #[ts(type = "AccountKind")]
    pub kind: AccountKind,
    /// If a PAT is not provided, we will use OAuth
    pub pat: Option<String>,
}

/// @category Type
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateAccountParams {
    pub id: AccountId,
    pub pat: Option<String>,
}
