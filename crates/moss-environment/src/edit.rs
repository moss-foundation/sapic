use joinerror::ResultExt;
use json_patch::PatchOperation;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem, RenameOptions};
use moss_hcl::HclResultExt;
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};
use tokio::sync::{RwLock, watch};

use crate::{configuration::SourceFile, environment::EnvironmentPath};

struct EnvironmentEditingState {
    abs_path: Arc<Path>,
    edit: JsonEdit,
}

pub(super) struct EnvironmentEditing {
    fs: Arc<dyn FileSystem>,
    state: RwLock<EnvironmentEditingState>,
    abs_path_tx: watch::Sender<EnvironmentPath>,
}

impl EnvironmentEditing {
    pub fn new(fs: Arc<dyn FileSystem>, abs_path_tx: watch::Sender<EnvironmentPath>) -> Self {
        let abs_path = abs_path_tx.borrow().full_path.clone();
        debug_assert!(abs_path.is_absolute());

        Self {
            fs,
            abs_path_tx,
            state: RwLock::new(EnvironmentEditingState {
                abs_path,
                edit: JsonEdit::new(),
            }),
        }
    }

    pub async fn rename(&self, ctx: &dyn AnyAsyncContext, new_name: &str) -> joinerror::Result<()> {
        let parent = self.abs_path_tx.borrow().parent.clone();
        let new_abs_path = EnvironmentPath::new(parent.join(new_name))
            .join_err::<()>("failed to create new environment path")?;

        let mut state_lock = self.state.write().await;
        self.fs
            .rename(
                ctx,
                &state_lock.abs_path,
                &new_abs_path.full_path,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;

        state_lock.abs_path = new_abs_path.full_path.clone();
        drop(state_lock);

        let _ = self.abs_path_tx.send(new_abs_path);

        Ok(())
    }

    pub async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;

        let rdr = self
            .fs
            .open_file(ctx, &state_lock.abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open file: {}", state_lock.abs_path.display())
            })?;

        let mut value: JsonValue = hcl::from_reader(rdr).join_err::<()>("failed to parse json")?;

        state_lock
            .edit
            .apply(&mut value, params)
            .join_err::<()>("failed to apply patches")?;

        let parsed: SourceFile = serde_json::from_value(value)?;
        let content = hcl::to_string(&parsed).join_err::<()>("failed to serialize json")?;
        self.fs
            .create_file_with(
                ctx,
                &state_lock.abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| {
                format!("failed to write file: {}", state_lock.abs_path.display())
            })?;

        Ok(())
    }
}
