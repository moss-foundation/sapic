use moss_fs::{FileSystem, FsResultExt};
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use rustc_hash::FxHashMap;
use sapic_base::{
    project::{manifest::ProjectManifest, types::primitives::ProjectId},
    workspace::types::primitives::WorkspaceId,
};
use std::{path::PathBuf, sync::Arc};

use crate::project::ProjectReader;

pub static KEY_PROJECT_PREFIX: &'static str = "project";

pub fn key_project_order(id: &ProjectId) -> String {
    format!("{KEY_PROJECT_PREFIX}.{id}.order")
}

pub struct CreateProjectParams {}

pub struct CloneProjectParams {}

pub struct ImportProjectParams {}

pub struct ExportProjectParams {}

pub struct ProjectItem {
    pub id: ProjectId,
    pub manifest: ProjectManifest,

    // DEPRECATED: we will get rid of this field in the future
    pub order: Option<isize>,
}

pub struct ProjectService {
    workspace_id: WorkspaceId,
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    reader: Arc<dyn ProjectReader>,
    storage: Arc<dyn KvStorage>,
}

impl ProjectService {
    pub fn new(
        workspace_id: WorkspaceId,
        abs_path: PathBuf,
        fs: Arc<dyn FileSystem>,
        reader: Arc<dyn ProjectReader>,
        storage: Arc<dyn KvStorage>,
    ) -> Self {
        Self {
            workspace_id,
            abs_path,
            fs,
            reader,
            storage,
        }
    }

    pub async fn create_project(&self, params: CreateProjectParams) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn clone_project(&self, params: CloneProjectParams) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn delete_project(&self, id: &ProjectId) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn archive_project(&self, id: &ProjectId) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn unarchive_project(&self, id: &ProjectId) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn import_project(&self, params: ImportProjectParams) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn export_project(
        &self,
        id: &ProjectId,
        params: ExportProjectParams,
    ) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn projects(&self) -> joinerror::Result<Vec<ProjectItem>> {
        let mut projects = Vec::new();

        let metadata = self
            .storage
            .get_batch_by_prefix(
                StorageScope::Workspace(self.workspace_id.inner()),
                KEY_PROJECT_PREFIX,
            )
            .await
            .unwrap_or_else(|e| {
                tracing::warn!(
                    "failed to fetch metadata from database when listing projects: {}",
                    e
                );
                Vec::new()
            })
            .into_iter()
            .collect::<FxHashMap<_, _>>();

        let mut read_dir = self
            .fs
            .read_dir(&self.abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to read directory `{}`", self.abs_path.display())
            })?;

        while let Some(entry) = read_dir.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let id_str = entry.file_name().to_string_lossy().to_string();
            let id: ProjectId = id_str.clone().into();

            let manifest = self.reader.read_manifest(&entry.path()).await?;
            projects.push(ProjectItem {
                id: id.clone(),
                manifest,
                order: metadata
                    .get(&key_project_order(&id))
                    .and_then(|v| serde_json::from_value(v.clone()).ok()),
            });
        }

        Ok(projects)
    }
}
