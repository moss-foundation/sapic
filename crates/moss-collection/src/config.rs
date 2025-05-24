use std::{path::Path, sync::Arc};

use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigModel {
    pub external_path: Option<Arc<Path>>,
}
