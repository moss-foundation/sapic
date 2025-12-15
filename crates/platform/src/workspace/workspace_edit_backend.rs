use crate::workspace::MANIFEST_FILE_NAME;
use async_trait::async_trait;
use joinerror::ResultExt;
use json_patch::{PatchOperation, ReplaceOperation};
use jsonptr::PointerBuf;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem};
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_core::context::AnyAsyncContext;
use sapic_system::workspace::{WorkspaceEditBackend, WorkspaceEditParams};
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

pub struct WorkspaceFsEditBackend {
    workspaces_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    edits: RwLock<JsonEdit>,
}

impl WorkspaceFsEditBackend {
    pub fn new(fs: Arc<dyn FileSystem>, workspaces_dir: PathBuf) -> Arc<Self> {
        Self {
            workspaces_dir,
            fs,
            edits: RwLock::new(JsonEdit::new()),
        }
        .into()
    }
}

#[async_trait]
impl WorkspaceEditBackend for WorkspaceFsEditBackend {
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &WorkspaceId,
        params: WorkspaceEditParams,
    ) -> joinerror::Result<()> {
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

        // No need to update the file if there's no patch
        if patches.is_empty() {
            return Ok(());
        }

        let abs_path = self
            .workspaces_dir
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
}

#[cfg(test)]
mod tests {
    use crate::workspace::{tests::MockStorage, workspace_service_fs::WorkspaceServiceFs};
    use moss_fs::RealFileSystem;
    use moss_storage2::KvStorage;
    use moss_testutils::random_name::random_string;
    use sapic_core::context::ArcContext;
    use sapic_system::workspace::WorkspaceServiceFs as WorkspaceServicePort;

    use super::*;

    // We need WorkspaceServiceFs and Storage to create a workspace for testing
    async fn setup_workspace_edit_test() -> (
        ArcContext,
        Arc<WorkspaceServiceFs>,
        Arc<dyn KvStorage>,
        Arc<WorkspaceFsEditBackend>,
        PathBuf,
    ) {
        let ctx = ArcContext::background();
        let test_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("data")
            .join(random_string(10));
        let tmp_path = test_path.join("tmp");
        let workspaces_dir = tmp_path.join("workspaces");

        tokio::fs::create_dir_all(&tmp_path).await.unwrap();
        tokio::fs::create_dir_all(&workspaces_dir).await.unwrap();

        let fs = Arc::new(RealFileSystem::new(&tmp_path));
        let workspace_fs = WorkspaceServiceFs::new(fs.clone(), workspaces_dir.clone());
        let storage = MockStorage::new();
        let edit = WorkspaceFsEditBackend::new(fs.clone(), workspaces_dir);
        (ctx, workspace_fs, storage, edit, test_path)
    }

    #[tokio::test]
    async fn test_edit_rename() {
        let (ctx, workspace_fs, storage, edit, test_path) = setup_workspace_edit_test().await;
        let id = WorkspaceId::new();
        let old_name = random_string(10);
        let new_name = random_string(10);

        workspace_fs
            .create_workspace(&ctx, &id, &old_name, storage.clone())
            .await
            .unwrap();

        edit.edit(
            &ctx,
            &id,
            WorkspaceEditParams {
                name: Some(new_name.clone()),
            },
        )
        .await
        .unwrap();

        let workspaces = workspace_fs.lookup_workspaces(&ctx).await.unwrap();
        assert_eq!(workspaces[0].name, new_name);

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_edit_nonexistent() {
        let (ctx, _workspace_fs, _storage, edit, test_path) = setup_workspace_edit_test().await;
        let id = WorkspaceId::new();

        let new_name = random_string(10);
        let result = edit
            .edit(
                &ctx,
                &id,
                WorkspaceEditParams {
                    name: Some(new_name.clone()),
                },
            )
            .await;

        assert!(result.is_err());

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }
}
