use moss_fs::{CreateOptions, FileSystem};
use moss_user::models::primitives::AccountId;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

pub const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ConfigFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_path: Option<PathBuf>,
}

pub(crate) struct Config {
    file: RwLock<ConfigFile>,
}

impl From<ConfigFile> for Config {
    fn from(file: ConfigFile) -> Self {
        Config {
            file: RwLock::new(file),
        }
    }
}

impl Config {
    pub async fn new(fs: &Arc<dyn FileSystem>, abs_path: &Path) -> joinerror::Result<Self> {
        let rdr = fs.open_file(&abs_path.join(CONFIG_FILE_NAME)).await?;
        let file: ConfigFile = serde_json::from_reader(rdr)?;

        Ok(Config {
            file: RwLock::new(file),
        })
    }

    pub async fn set_account_id(
        &self,
        fs: &Arc<dyn FileSystem>,
        abs_path: &Path,
        account_id: AccountId,
    ) -> joinerror::Result<()> {
        let mut file_lock = self.file.write().await;
        let mut conf_clone = file_lock.clone();

        conf_clone.account_id = Some(account_id);

        fs.create_file_with(
            &abs_path.join(CONFIG_FILE_NAME),
            serde_json::to_string(&conf_clone)?.as_bytes(),
            CreateOptions {
                overwrite: true,
                ignore_if_exists: false,
            },
        )
        .await?;

        *file_lock = conf_clone;

        Ok(())
    }
}
