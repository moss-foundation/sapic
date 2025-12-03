use sapic_base::user::types::{
    AccountInfo,
    primitives::{AccountId, AccountKind},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

//
// List User Accounts
//

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListUserAccountsOutput {
    #[ts(type = "AccountInfo[]")]
    pub accounts: Vec<AccountInfo>,
}

//
// Add User Account
//

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct AddUserAccountInput {
    pub host: String,
    #[ts(type = "AccountKind")]
    pub kind: AccountKind,
    /// If a PAT is not provided, we will use OAuth
    pub pat: Option<String>,
}

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateUserAccountInput {
    pub id: AccountId,
    pub pat: Option<String>,
}

//
// Remove User Account
//

/// @category Operation
#[derive(Debug, Validate, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct RemoveUserAccountInput {
    pub id: AccountId,
}
