use std::{path::PathBuf, sync::Arc};

use joinerror::ResultExt;
use json_patch::PatchOperation;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem};
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use tokio::sync::RwLock;

struct WorkspaceEditingState {
    edit: JsonEdit,
}

pub(super) struct WorkspaceEdit {
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    state: RwLock<WorkspaceEditingState>,
}

impl WorkspaceEdit {
    pub fn new(fs: Arc<dyn FileSystem>, abs_path: PathBuf) -> Self {
        Self {
            fs,
            abs_path,
            state: RwLock::new(WorkspaceEditingState {
                edit: JsonEdit::new(),
            }),
        }
    }

    pub async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()> {
        let rdr = self
            .fs
            .open_file(ctx, &self.abs_path)
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", self.abs_path.display()))?;

        let mut value: JsonValue =
            serde_json::from_reader(rdr).join_err::<()>("failed to parse json")?;

        let mut state_lock = self.state.write().await;
        state_lock
            .edit
            .apply(&mut value, params)
            .join_err::<()>("failed to apply patches")?;

        let content = serde_json::to_string(&value).join_err::<()>("failed to serialize json")?;

        self.fs
            .create_file_with(
                ctx,
                &self.abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to write file: {}", self.abs_path.display()))?;

        Ok(())
    }
}
