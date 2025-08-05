use joinerror::ResultExt;
use json_patch::PatchOperation;
use moss_contentmodel::json::JsonEdit;
use moss_fs::{CreateOptions, FileSystem, error::FsResultExt};
use serde_json::Value as JsonValue;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{RwLock, watch};

use crate::environment::EnvironmentPath;

struct EditingState {
    abs_path: PathBuf,
    edit: JsonEdit,
}

pub struct EnvironmentEditing {
    fs: Arc<dyn FileSystem>,
    state: RwLock<EditingState>,
    abs_path_tx: watch::Sender<EnvironmentPath>,
}

impl EnvironmentEditing {
    pub fn new(fs: Arc<dyn FileSystem>, abs_path_tx: watch::Sender<EnvironmentPath>) -> Self {
        let abs_path = abs_path_tx.borrow().to_path_buf();

        Self {
            fs,
            abs_path_tx,
            state: RwLock::new(EditingState {
                abs_path,
                edit: JsonEdit::new(),
            }),
        }
    }

    // pub async fn rename(&self, new_name: &str) -> joinerror::Result<()> {

    // }

    pub async fn edit(&self, params: &[PatchOperation]) -> joinerror::Result<()> {
        let state_lock = self.state.write().await;

        let rdr = self
            .fs
            .open_file(&state_lock.abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open file: {}", state_lock.abs_path.display())
            })?;

        let mut value: JsonValue =
            serde_json::from_reader(rdr).join_err::<()>("failed to parse json")?;

        self.state
            .write()
            .await
            .edit
            .apply(&mut value, params)
            .join_err::<()>("failed to apply patches")?;

        let content = serde_json::to_string(&value).join_err::<()>("failed to serialize json")?;
        self.fs
            .create_file_with(
                &state_lock.abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| {
                format!("failed to create file: {}", state_lock.abs_path.display())
            })?;

        Ok(())
    }
}
