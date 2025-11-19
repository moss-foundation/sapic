use async_trait::async_trait;
use joinerror::ResultExt;
use moss_common::continue_if_err;
use moss_fs::{FileSystem, FsResultExt};
use sapic_base::workspace::{manifest::ManifestFile, types::primitives::WorkspaceId};
use sapic_system::workspace::{
    WorkspaceDiscoverer as WorkspaceDiscovererPort, types::DiscoveredWorkspace,
};
use std::{path::PathBuf, sync::Arc};

const MANIFEST_FILE_NAME: &str = "Sapic.json";

pub struct WorkspaceDiscoverer {
    workspaces_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
}

impl WorkspaceDiscoverer {
    pub fn new(fs: Arc<dyn FileSystem>, workspaces_dir: PathBuf) -> Self {
        Self { fs, workspaces_dir }
    }
}

#[async_trait]
impl WorkspaceDiscovererPort for WorkspaceDiscoverer {
    async fn discover_workspaces(&self) -> joinerror::Result<Vec<DiscoveredWorkspace>> {
        let mut read_dir = self.fs.read_dir(&self.workspaces_dir).await?;
        let mut workspaces = vec![];

        while let Some(entry) = read_dir.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let id_str = entry.file_name().to_string_lossy().to_string();
            let id: WorkspaceId = id_str.into();

            let path = entry.path().join(MANIFEST_FILE_NAME);

            let manifest = continue_if_err!(
                async {
                    let rdr = self.fs.open_file(&path).await.join_err_with::<()>(|| {
                        format!("failed to open manifest file: {}", path.display())
                    })?;

                    let file: ManifestFile =
                        serde_json::from_reader(rdr).join_err_with::<()>(|| {
                            format!("failed to parse manifest file: {}", path.display())
                        })?;

                    Ok(file)
                },
                |err: joinerror::Error| {
                    tracing::warn!("failed to parse manifest file: {}", err);
                }
            );

            workspaces.push(DiscoveredWorkspace {
                id,
                name: manifest.name,
                abs_path: entry.path(),
            });
        }

        Ok(workspaces)
    }
}
