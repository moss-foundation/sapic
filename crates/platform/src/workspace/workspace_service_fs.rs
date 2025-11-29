use async_trait::async_trait;
use joinerror::ResultExt;
use moss_common::continue_if_err;
use moss_environment::builder::{CreateEnvironmentParams, EnvironmentBuilder};
use moss_fs::{CreateOptions, FileSystem, FsResultExt, RemoveOptions};
use moss_storage2::KvStorage;
use sapic_base::{
    environment::PredefinedEnvironment,
    errors::AlreadyExists,
    workspace::{manifest::ManifestFile, types::primitives::WorkspaceId},
};
use sapic_system::workspace::{LookedUpWorkspace, WorkspaceServiceFs as WorkspaceServiceFsPort};
use std::{cell::LazyCell, path::PathBuf, sync::Arc};

use crate::workspace::MANIFEST_FILE_NAME;

const WORKSPACE_DIRS: &[&str] = &["projects", "environments"];
const PREDEFINED_ENVIRONMENTS: LazyCell<Vec<PredefinedEnvironment>> = LazyCell::new(|| {
    vec![PredefinedEnvironment {
        name: "Globals".to_string(),
        color: Some("#3574F0".to_string()),
    }]
});

pub struct WorkspaceServiceFs {
    workspaces_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
}

impl WorkspaceServiceFs {
    pub fn new(fs: Arc<dyn FileSystem>, workspaces_dir: PathBuf) -> Self {
        Self { fs, workspaces_dir }
    }
}

#[async_trait]
impl WorkspaceServiceFsPort for WorkspaceServiceFs {
    async fn lookup_workspaces(&self) -> joinerror::Result<Vec<LookedUpWorkspace>> {
        let mut read_dir = self.fs.read_dir(&self.workspaces_dir).await?;
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
                    let rdr = self.fs.open_file(&abs_path).await.join_err_with::<()>(|| {
                        format!("failed to open manifest file: {}", abs_path.display())
                    })?;

                    let file: ManifestFile =
                        serde_json::from_reader(rdr).join_err_with::<()>(|| {
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

    async fn create_workspace(
        &self,
        id: &WorkspaceId,
        name: &str,

        // FIXME: Passing the store here is a temporary solution until we move the environment creation out of this function.
        storage: Arc<dyn KvStorage>,
    ) -> joinerror::Result<PathBuf> {
        let abs_path = self.workspaces_dir.join(id.as_str());
        if abs_path.exists() {
            return Err(joinerror::Error::new::<AlreadyExists>(id.as_str()));
        }

        let mut rb = self.fs.start_rollback().await?;

        self.fs
            .create_dir_with_rollback(&mut rb, &abs_path)
            .await
            .join_err::<()>("failed to create workspace directory")?;

        for dir in WORKSPACE_DIRS {
            self.fs
                .create_dir_with_rollback(&mut rb, &abs_path.join(dir))
                .await
                .join_err::<()>("failed to create workspace directory")?;
        }

        for env in PREDEFINED_ENVIRONMENTS.iter() {
            EnvironmentBuilder::new(id.inner(), self.fs.clone(), storage.clone())
                .initialize(CreateEnvironmentParams {
                    name: env.name.clone(),
                    abs_path: &abs_path.join("environments"),
                    color: env.color.clone(),
                    variables: vec![],
                })
                .await
                .join_err_with::<()>(|| {
                    format!("failed to initialize environment `{}`", env.name)
                })?;
        }

        self.fs
            .create_file_with_content_with_rollback(
                &mut rb,
                &abs_path.join(MANIFEST_FILE_NAME),
                serde_json::to_string(&ManifestFile {
                    name: name.to_string(),
                })?
                .as_bytes(),
                CreateOptions::default(),
            )
            .await
            .join_err::<()>("failed to create manifest file")?;

        Ok(abs_path)
    }

    async fn delete_workspace(&self, id: &WorkspaceId) -> joinerror::Result<Option<PathBuf>> {
        let abs_path = self.workspaces_dir.join(id.as_str());
        if !abs_path.exists() {
            return Ok(None);
        }

        self.fs
            .remove_dir(
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
