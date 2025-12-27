use async_trait::async_trait;
use joinerror::ResultExt;
use json_patch::{PatchOperation, ReplaceOperation};
use jsonptr::PointerBuf;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem};
use sapic_base::project::{
    config::CONFIG_FILE_NAME, manifest::MANIFEST_FILE_NAME, types::primitives::ProjectId,
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::project::{ProjectConfigEditParams, ProjectEditBackend, ProjectEditParams};
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

pub struct ProjectFsEditBackend {
    projects_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    edits: RwLock<JsonEdit>,
}

impl ProjectFsEditBackend {
    pub fn new(fs: Arc<dyn FileSystem>, projects_dir: PathBuf) -> Arc<Self> {
        Arc::new(Self {
            projects_dir,
            fs,
            edits: RwLock::new(JsonEdit::new()),
        })
    }
}

#[async_trait]
impl ProjectEditBackend for ProjectFsEditBackend {
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ProjectEditParams,
    ) -> joinerror::Result<()> {
        // TODO: Implement relinking and unlinking remote repo when the user update it
        // Right now I can't test git functionality properly since the frontend has not updated the project creation with git logic
        // So I'll skip repo update for now

        let mut patches = Vec::new();

        if let Some(new_name) = params.name {
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/name") },
                    value: JsonValue::String(new_name),
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));
        }

        if patches.is_empty() {
            return Ok(());
        }

        let abs_path = self
            .projects_dir
            .join(id.to_string())
            .join(MANIFEST_FILE_NAME);
        let rdr = self
            .fs
            .open_file(ctx, &abs_path)
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

    async fn edit_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ProjectConfigEditParams,
    ) -> joinerror::Result<()> {
        let mut patches = Vec::new();

        if let Some(archived) = params.archived {
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/archived") },
                    value: JsonValue::Bool(archived),
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));
        }

        if patches.is_empty() {
            return Ok(());
        }

        let abs_path = self
            .projects_dir
            .join(id.to_string())
            .join(CONFIG_FILE_NAME);
        let rdr = self
            .fs
            .open_file(ctx, &abs_path)
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
