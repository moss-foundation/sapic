use async_trait::async_trait;
use joinerror::ResultExt;
use moss_common::continue_if_err;
use moss_environment::builder::{CreateEnvironmentParams, EnvironmentBuilder};
use moss_fs::{CreateOptions, FileSystem, RemoveOptions};
use moss_storage2::KvStorage;
use sapic_base::{
    environment::PredefinedEnvironment,
    errors::AlreadyExists,
    workspace::{manifest::WorkspaceManifest, types::primitives::WorkspaceId},
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::workspace::{LookedUpWorkspace, WorkspaceServiceFs as WorkspaceServiceFsPort};
use std::{cell::LazyCell, path::PathBuf, sync::Arc};

use crate::workspace::MANIFEST_FILE_NAME;

const WORKSPACE_DIRS: &[&str] = &["projects", "environments"];

pub struct WorkspaceServiceFs {
    workspaces_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
}

impl WorkspaceServiceFs {
    pub fn new(fs: Arc<dyn FileSystem>, workspaces_dir: PathBuf) -> Arc<Self> {
        Self { fs, workspaces_dir }.into()
    }
}

#[async_trait]
impl WorkspaceServiceFsPort for WorkspaceServiceFs {
    async fn lookup_workspaces(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<LookedUpWorkspace>> {
        let mut read_dir = self.fs.read_dir(ctx, &self.workspaces_dir).await?;
        let mut workspaces = vec![];

        while let Some(entry) = read_dir.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let id_str = entry.file_name().to_string_lossy().to_string();
            let id: WorkspaceId = id_str.into();
            let abs_path = entry.path().join(MANIFEST_FILE_NAME);

            let manifest = continue_if_err!(
                async {
                    let rdr = self
                        .fs
                        .open_file(ctx, &abs_path)
                        .await
                        .join_err_with::<()>(|| {
                            format!("failed to open manifest file: {}", abs_path.display())
                        })?;

                    let file: WorkspaceManifest = serde_json::from_reader(rdr)
                        .join_err_with::<()>(|| {
                            format!("failed to parse manifest file: {}", abs_path.display())
                        })?;

                    Ok(file)
                },
                |err: joinerror::Error| {
                    tracing::warn!("failed to parse manifest file: {}", err);
                }
            );

            workspaces.push(LookedUpWorkspace {
                id,
                name: manifest.name,
                abs_path: entry.path(),
            });
        }

        Ok(workspaces)
    }

    // FIXME: This is still not correctly deleting all the files when a workspace is open
    // The database files and the folders will not be removed
    // It doesn't crash the app but still something we need to solve
    async fn create_workspace(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &WorkspaceId,
        name: &str,
    ) -> joinerror::Result<PathBuf> {
        let abs_path = self.workspaces_dir.join(id.as_str());
        if abs_path.exists() {
            return Err(joinerror::Error::new::<AlreadyExists>(id.as_str()));
        }

        let mut rb = self.fs.start_rollback(ctx).await?;

        self.fs
            .create_dir_with_rollback(ctx, &mut rb, &abs_path)
            .await
            .join_err::<()>("failed to create workspace directory")?;

        for dir in WORKSPACE_DIRS {
            self.fs
                .create_dir_with_rollback(ctx, &mut rb, &abs_path.join(dir))
                .await
                .join_err::<()>("failed to create workspace directory")?;
        }

        self.fs
            .create_file_with_content_with_rollback(
                ctx,
                &mut rb,
                &abs_path.join(MANIFEST_FILE_NAME),
                serde_json::to_string(&WorkspaceManifest {
                    name: name.to_string(),
                })?
                .as_bytes(),
                CreateOptions::default(),
            )
            .await
            .join_err::<()>("failed to create manifest file")?;

        Ok(abs_path)
    }

    async fn delete_workspace(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &WorkspaceId,
    ) -> joinerror::Result<Option<PathBuf>> {
        let abs_path = self.workspaces_dir.join(id.as_str());
        if !abs_path.exists() {
            return Ok(None);
        }

        self.fs
            .remove_dir(
                ctx,
                &abs_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to delete workspace `{}`", id.as_str()))?;

        Ok(Some(abs_path))
    }
}

#[cfg(test)]
mod tests {
    use moss_fs::RealFileSystem;
    use moss_storage2::KvStorage;
    use moss_testutils::random_name::random_string;
    use sapic_base::workspace::types::primitives::WorkspaceId;

    use crate::workspace::tests::MockStorage;
    use sapic_core::context::ArcContext;
    use std::{path::PathBuf, sync::Arc};

    use super::*;

    async fn setup_test_workspace_service_fs() -> (
        ArcContext,
        Arc<WorkspaceServiceFs>,
        Arc<dyn KvStorage>,
        PathBuf,
    ) {
        let ctx = ArcContext::background();
        let test_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("data")
            .join(random_string(10));
        let tmp_path = test_path.join("tmp");
        let workspaces_dir = test_path.join("workspaces");

        tokio::fs::create_dir_all(&tmp_path).await.unwrap();
        tokio::fs::create_dir_all(&workspaces_dir).await.unwrap();

        let fs = Arc::new(RealFileSystem::new(&tmp_path));
        let workspace_fs = WorkspaceServiceFs::new(fs, workspaces_dir);
        let storage = MockStorage::new();

        (ctx, workspace_fs, storage, test_path)
    }

    #[tokio::test]
    async fn test_create_workspace_normal() {
        let (ctx, service_fs, storage, test_path) = setup_test_workspace_service_fs().await;
        let id = WorkspaceId::new();

        let workspace_path = service_fs
            .create_workspace(&ctx, &id, &random_string(10))
            .await
            .unwrap();

        assert!(workspace_path.exists());
        for dir in WORKSPACE_DIRS {
            assert!(workspace_path.join(dir).exists());
        }
        assert!(workspace_path.join(MANIFEST_FILE_NAME).exists());

        drop(service_fs);
        drop(storage);
        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    // This should work since workspace name has nothing to do with filesystem
    #[tokio::test]
    async fn test_create_workspace_empty_name() {
        let (ctx, service_fs, storage, test_path) = setup_test_workspace_service_fs().await;
        let id = WorkspaceId::new();

        let workspace_path = service_fs.create_workspace(&ctx, &id, "").await.unwrap();

        assert!(workspace_path.exists());
        for dir in WORKSPACE_DIRS {
            assert!(workspace_path.join(dir).exists());
        }
        assert!(workspace_path.join(MANIFEST_FILE_NAME).exists());

        drop(service_fs);
        drop(storage);
        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_workspace_already_exists() {
        let (ctx, service_fs, storage, test_path) = setup_test_workspace_service_fs().await;
        let id = WorkspaceId::new();

        service_fs
            .create_workspace(&ctx, &id, &random_string(10))
            .await
            .unwrap();

        let result = service_fs
            .create_workspace(&ctx, &id, &random_string(10))
            .await;

        assert!(result.is_err());

        drop(service_fs);
        drop(storage);
        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_workspace_success() {
        let (ctx, service_fs, storage, test_path) = setup_test_workspace_service_fs().await;
        let id = WorkspaceId::new();

        let workspace_path = service_fs
            .create_workspace(&ctx, &id, &random_string(10))
            .await
            .unwrap();

        service_fs.delete_workspace(&ctx, &id).await.unwrap();

        assert!(!workspace_path.exists());
        drop(service_fs);
        drop(storage);
        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    // Deleting a nonexistent workspace should be handled gracefully
    #[tokio::test]
    async fn test_delete_workspace_nonexistent() {
        let (ctx, service_fs, storage, test_path) = setup_test_workspace_service_fs().await;
        let id = WorkspaceId::new();

        let result = service_fs.delete_workspace(&ctx, &id).await.unwrap();

        // No path will be returned if we delete a non-existent workspace
        assert!(result.is_none());
        drop(service_fs);
        drop(storage);
        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn lookup_workspaces_empty() {
        let (ctx, service_fs, storage, test_path) = setup_test_workspace_service_fs().await;

        let workspaces = service_fs.lookup_workspaces(&ctx).await.unwrap();
        assert!(workspaces.is_empty());

        drop(service_fs);
        drop(storage);
        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn lookup_workspaces_normal() {
        let (ctx, service_fs, storage, test_path) = setup_test_workspace_service_fs().await;
        let id = WorkspaceId::new();
        let name = random_string(10);
        let workspace_path = service_fs.create_workspace(&ctx, &id, &name).await.unwrap();

        let workspaces = service_fs.lookup_workspaces(&ctx).await.unwrap();

        assert_eq!(workspaces.len(), 1);
        assert_eq!(workspaces[0].id, id);
        assert_eq!(workspaces[0].name, name);
        assert_eq!(workspaces[0].abs_path, workspace_path);

        drop(service_fs);
        drop(storage);
        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn lookup_workspaces_after_deletion() {
        let (ctx, service_fs, storage, test_path) = setup_test_workspace_service_fs().await;
        let id = WorkspaceId::new();
        let name = random_string(10);
        service_fs.create_workspace(&ctx, &id, &name).await.unwrap();

        service_fs.delete_workspace(&ctx, &id).await.unwrap();

        let workspaces = service_fs.lookup_workspaces(&ctx).await.unwrap();
        assert!(workspaces.is_empty());

        drop(service_fs);
        drop(storage);
        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }
}
