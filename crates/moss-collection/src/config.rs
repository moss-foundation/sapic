use moss_user::models::primitives::AccountId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_path: Option<PathBuf>,
}
