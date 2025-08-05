use crate::{
    manifest::{MANIFEST_FILE_NAME, ManifestModel},
    models::primitives::CollectionId,
    services::{
        collection_service::CollectionService, environment_service::EnvironmentService,
        layout_service::LayoutService, storage_service::StorageService,
    },
};
use anyhow::Result;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::AppRuntime;
use moss_collection::Collection;
use moss_environment::{AnyEnvironment, Environment, models::primitives::EnvironmentId};
use moss_file::json::JsonFileHandle;
use moss_fs::FileSystem;
use moss_git::GitAuthAgent;
use moss_git_hosting_provider::{github::client::GitHubClient, gitlab::client::GitLabClient};
use std::{path::Path, sync::Arc};

pub struct WorkspaceSummary {
    pub manifest: ManifestModel,
}

#[derive(Clone)]
pub struct WorkspaceModifyParams {
    pub name: Option<String>,
}

pub trait AnyWorkspace<R: AppRuntime> {
    type Collection;
    type Environment: AnyEnvironment<R>;
}

pub struct Workspace<R: AppRuntime> {
    pub(super) abs_path: Arc<Path>,

    #[allow(dead_code)]
    pub(super) activity_indicator: ActivityIndicator<R::EventLoop>,
    #[allow(dead_code)]
    pub(super) manifest: JsonFileHandle<ManifestModel>,

    pub(super) layout_service: LayoutService<R>,
    pub(super) collection_service: CollectionService<R>,
    pub(super) environment_service: EnvironmentService<R>,
    pub(super) storage_service: Arc<StorageService<R>>,

    // TODO: Refine the management of git provider clients
    pub(super) github_client: Arc<GitHubClient>,
    pub(super) gitlab_client: Arc<GitLabClient>,
}

impl<R: AppRuntime> AnyWorkspace<R> for Workspace<R> {
    type Collection = Collection<R>;
    type Environment = Environment<R>;
}

impl<R: AppRuntime> Workspace<R> {
    pub fn abs_path(&self) -> &Path {
        &self.abs_path
    }

    // TODO: return Option<Arc<Collection<R>>>
    pub async fn collection(&self, id: &CollectionId) -> joinerror::Result<Arc<Collection<R>>> {
        self.collection_service.collection(id).await
    }

    pub async fn environment(&self, id: &EnvironmentId) -> Option<Arc<Environment<R>>> {
        self.environment_service.environment(id).await
    }

    // INFO: This will probably be moved to EditService in the future.
    pub async fn modify(&self, params: WorkspaceModifyParams) -> Result<()> {
        if params.name.is_some() {
            self.manifest
                .edit(
                    |model| {
                        model.name = params.name.unwrap();
                        Ok(())
                    },
                    |model| {
                        serde_json::to_string(model).map_err(|err| {
                            anyhow::anyhow!("Failed to serialize JSON file: {}", err)
                        })
                    },
                )
                .await?;
        }
        Ok(())
    }

    // TODO: Move out of the Workspace struct
    pub async fn summary(fs: Arc<dyn FileSystem>, abs_path: &Path) -> Result<WorkspaceSummary> {
        let manifest = JsonFileHandle::load(fs, &abs_path.join(MANIFEST_FILE_NAME)).await?;
        Ok(WorkspaceSummary {
            manifest: manifest.model().await,
        })
    }

    pub async fn manifest(&self) -> ManifestModel {
        self.manifest.model().await
    }
}

#[cfg(any(test, feature = "integration-tests"))]
impl<R: AppRuntime> Workspace<R> {
    pub fn db(&self) -> &Arc<dyn moss_storage::WorkspaceStorage<R::AsyncContext>> {
        self.storage_service.storage()
    }
}
