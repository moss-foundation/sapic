use std::{path::Path, sync::Arc};

use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigModel {
    pub external_path: Option<Arc<Path>>,
}
