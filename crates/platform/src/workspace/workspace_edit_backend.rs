use async_trait::async_trait;
use joinerror::ResultExt;
use json_patch::{PatchOperation, ReplaceOperation};
use jsonptr::PointerBuf;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem, FsResultExt};
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_system::workspace::{WorkspaceEditBackend, WorkspaceEditParams};
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::workspace::MANIFEST_FILE_NAME;

pub struct WorkspaceFsEditBackend {
    workspaces_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    edits: RwLock<JsonEdit>,
}

impl WorkspaceFsEditBackend {
    pub fn new(fs: Arc<dyn FileSystem>, workspaces_dir: PathBuf) -> Self {
        Self {
            workspaces_dir,
            fs,
            edits: RwLock::new(JsonEdit::new()),
        }
    }
}

#[async_trait]
impl WorkspaceEditBackend for WorkspaceFsEditBackend {
    async fn edit(&self, id: &WorkspaceId, params: WorkspaceEditParams) -> joinerror::Result<()> {
        let mut patches = Vec::new();

        if let Some(new_name) = params.name {
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/name") },
                    value: JsonValue::String(new_name),
                }),
                EditOptions {
                    ignore_if_not_exists: false,
                    create_missing_segments: false,
                },
            ));
        }

        let abs_path = self
            .workspaces_dir
            .join(id.to_string())
            .join(MANIFEST_FILE_NAME);
        let rdr = self
            .fs
            .open_file(&abs_path)
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;

        let mut value: JsonValue =
            serde_json::from_reader(rdr).join_err::<()>("failed to parse json")?;

        let mut edits_lock = self.edits.write().await;
        edits_lock
            .apply(&mut value, &patches)
            .join_err::<()>("failed to apply patches")?;

        let content = serde_json::to_string(&value).join_err::<()>("failed to serialize json")?;

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
