use joinerror::{Error, OptionExt, ResultExt};
use json_patch::PatchOperation;
use moss_api::errors::InvalidInput;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem, FsResultExt, RenameOptions};
use moss_hcl::HclResultExt;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};
use tokio::sync::{RwLock, watch};

use crate::{
    errors::{ErrorAlreadyExists, ErrorInternal},
    spec::EntryModel,
};

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

    pub async fn rename(&self, abs_path: &Path, from: &Path, to: &Path) -> joinerror::Result<()> {
        debug_assert!(abs_path.is_absolute());
        debug_assert!(from.is_relative());
        debug_assert!(to.is_relative());

        // let path = self.path_tx.borrow();
        // let parent = path.parent().ok_or_join_err_with::<()>(|| {
        //     format!(
        //         "failed to get parent path of {}",
        //         abs_path.join(path.as_ref()).display()
        //     )
        // })?;

        // let new_path: Arc<Path> = parent.join(new_name).into();

        // let mut state_lock = self.state.write().await;
        // self.fs
        //     .rename(
        //         &abs_path.join(&state_lock.path),
        //         &abs_path.join(&new_path),
        //         RenameOptions {
        //             overwrite: true,
        //             ignore_if_exists: false,
        //         },
        //     )
        //     .await?;

        // On Windows and macOS, file/directory names are case-preserving but insensitive
        // If the from and to path differs only with different casing of the filename
        // The rename should still succeed

        let abs_from = abs_path.join(from);
        let abs_to = abs_path.join(to);

        let old_name_lower = from
            .file_name()
            .map(|name| name.to_string_lossy().to_lowercase())
            .ok_or_join_err::<ErrorInternal>("invalid old file name")?;
        let new_name_lower = to
            .file_name()
            .map(|name| name.to_string_lossy().to_lowercase())
            .ok_or_join_err::<ErrorInternal>("invalid new file name")?;

        let recasing_only =
            old_name_lower == new_name_lower && abs_from.parent() == abs_to.parent();

        if abs_to.exists() && !recasing_only {
            return Err(Error::new::<ErrorAlreadyExists>(format!(
                "entry already exists: {}",
                to.display()
            )));
        }

        let mut state_lock = self.state.write().await;
        self.fs
            .rename(
                &abs_from,
                &abs_to,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;

        let new_path: Arc<Path> = to.into();

        state_lock.path = new_path.clone();
        drop(state_lock);

        let _ = self.path_tx.send(new_path);

        Ok(())
    }

    pub async fn edit(
        &self,
        abs_path: &Path,
        params: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()> {
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

        let parsed: EntryModel = serde_json::from_value(value)?;
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
