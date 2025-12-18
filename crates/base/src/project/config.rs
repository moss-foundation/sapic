use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::user::types::primitives::AccountId;

pub const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub archived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
}
