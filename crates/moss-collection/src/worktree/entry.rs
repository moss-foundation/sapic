use joinerror::ResultExt;
use json_patch::PatchOperation;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem, FsResultExt, RenameOptions};
use moss_hcl::HclResultExt;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};
use tokio::sync::{RwLock, watch};

struct EntryEditingState {
    path: Arc<Path>,
    edit: JsonEdit,
}

pub(super) struct EntryEditing {
    fs: Arc<dyn FileSystem>,
    state: RwLock<EntryEditingState>,
    path_tx: watch::Sender<Arc<Path>>,
}

impl EntryEditing {
    pub fn new(fs: Arc<dyn FileSystem>, path_tx: watch::Sender<Arc<Path>>) -> Self {
        let path = path_tx.borrow().clone();

        Self {
            fs,
            path_tx,
            state: RwLock::new(EntryEditingState {
                path,
                edit: JsonEdit::new(),
            }),
        }
    }

    pub async fn rename(&self, abs_path: &Path, new_name: &str) -> joinerror::Result<()> {
        let parent = self.path_tx.borrow().clone();
        let new_path: Arc<Path> = parent.join(new_name).into();

        let mut state_lock = self.state.write().await;
        self.fs
            .rename(
                &abs_path.join(&state_lock.path),
                &abs_path.join(&new_path),
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;

        state_lock.path = new_path.clone();
        drop(state_lock);

        let _ = self.path_tx.send(new_path);

        Ok(())
    }

    pub async fn edit<T>(
        &self,
        abs_path: &Path,
        params: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        let mut state_lock = self.state.write().await;

        let abs_path = abs_path.join(&state_lock.path);
        let rdr = self
            .fs
            .open_file(&abs_path)
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;

        let mut value: JsonValue = hcl::from_reader(rdr).join_err::<()>("failed to parse json")?;

        state_lock
            .edit
            .apply(&mut value, params)
            .join_err::<()>("failed to apply patches")?;

        let parsed: T = serde_json::from_value(value)?;
        let content = hcl::to_string(&parsed).join_err::<()>("failed to serialize json")?;
        self.fs
            .create_file_with(
                &abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to write file: {}", abs_path.display()))?;

        Ok(())
    }
}
