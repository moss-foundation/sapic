use moss_user::models::types::AccountInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProfileFile {
    pub name: String,
    pub is_default: bool,
    pub accounts: Vec<AccountInfo>,
}
