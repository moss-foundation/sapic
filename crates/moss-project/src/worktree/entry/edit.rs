use crate::{
    errors::{ErrorAlreadyExists, ErrorInternal},
    worktree::entry::model::EntryModel,
};
use joinerror::{Error, OptionExt, ResultExt};
use json_patch::PatchOperation;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem, RenameOptions};
use moss_hcl::HclResultExt;
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};
use tokio::sync::{RwLock, watch};

struct EntryEditingState {
    path: Arc<Path>,
    edit: JsonEdit,
}

pub(crate) struct EntryEditing {
    fs: Arc<dyn FileSystem>,
    state: RwLock<EntryEditingState>,
    path_tx: watch::Sender<Arc<Path>>,
    // We need to know if it's item or dir config
    config_filename: String,
}

impl EntryEditing {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        path_tx: watch::Sender<Arc<Path>>,
        file_name: &str,
    ) -> Self {
        let path = path_tx.borrow().clone().into();

        Self {
            fs,
            path_tx,
            state: RwLock::new(EntryEditingState {
                path,
                edit: JsonEdit::new(),
            }),
            config_filename: file_name.to_string(),
        }
    }

    pub async fn rename(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
        from: &Path,
        to: &Path,
    ) -> joinerror::Result<()> {
        debug_assert!(abs_path.is_absolute());
        debug_assert!(from.is_relative());
        debug_assert!(to.is_relative());

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
                ctx,
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
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
        params: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;

        let abs_path = abs_path.join(&state_lock.path.join(&self.config_filename));
        let rdr = self
            .fs
            .open_file(ctx, &abs_path)
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
                ctx,
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
