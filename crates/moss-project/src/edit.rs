use joinerror::ResultExt;
use json_patch::PatchOperation;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem};
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

struct CollectionEditingState {
    edit: JsonEdit,
}

pub(super) struct ProjectEdit {
    fs: Arc<dyn FileSystem>,
    state: RwLock<CollectionEditingState>,
    manifest_abs_path: PathBuf,
}

impl ProjectEdit {
    pub fn new(fs: Arc<dyn FileSystem>, manifest_abs_path: PathBuf) -> Self {
        Self {
            fs,
            state: RwLock::new(CollectionEditingState {
                edit: JsonEdit::new(),
            }),
            manifest_abs_path,
        }
    }

    pub async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()> {
        let rdr = self
            .fs
            .open_file(ctx, &self.manifest_abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open file: {}", self.manifest_abs_path.display())
            })?;

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
                &self.manifest_abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| {
                format!("failed to write file: {}", self.manifest_abs_path.display())
            })?;

        Ok(())
    }
}
