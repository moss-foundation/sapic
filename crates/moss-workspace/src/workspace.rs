use anyhow::Result;
use joinerror::ResultExt;
use json_patch::{PatchOperation, ReplaceOperation};
use jsonptr::PointerBuf;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::AppRuntime;
use moss_collection::Collection;
use moss_environment::{AnyEnvironment, Environment, models::primitives::EnvironmentId};
use moss_file::json::JsonFileHandle;
use moss_fs::{FileSystem, FsResultExt};
use moss_git::GitAuthAgent;
use moss_git_hosting_provider::{github::client::GitHubClient, gitlab::client::GitLabClient};
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};

use crate::{
    edit::WorkspaceEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    models::primitives::CollectionId,
    services::{
        collection_service::CollectionService, environment_service::EnvironmentService,
        layout_service::LayoutService, storage_service::StorageService,
    },
};

pub struct WorkspaceSummary {
    pub name: String,
}

impl WorkspaceSummary {
    pub async fn new(fs: &Arc<dyn FileSystem>, abs_path: &Path) -> joinerror::Result<Self> {
        debug_assert!(abs_path.is_absolute());

        let manifest_path = abs_path.join(MANIFEST_FILE_NAME);

        let rdr = fs.open_file(&manifest_path).await.join_err_with::<()>(|| {
            format!("failed to open manifest file: {}", manifest_path.display())
        })?;

        let manifest: ManifestFile = serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!("failed to parse manifest file: {}", manifest_path.display())
        })?;

        Ok(WorkspaceSummary {
            name: manifest.name,
        })
    }
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
    pub(super) edit: WorkspaceEdit,
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

    pub async fn collection(&self, id: &CollectionId) -> Option<Arc<Collection<R>>> {
        self.collection_service.collection(id).await
    }

    pub async fn environment(&self, id: &EnvironmentId) -> Option<Arc<Environment<R>>> {
        self.environment_service.environment(id).await
    }

    pub async fn modify(&self, params: WorkspaceModifyParams) -> Result<()> {
        let mut patches = Vec::new();

        if let Some(new_name) = params.name {
            patches.push(PatchOperation::Replace(ReplaceOperation {
                path: unsafe { PointerBuf::new_unchecked("/name") },
                value: JsonValue::String(new_name),
            }));
        }

        self.edit
            .edit(&patches)
            .await
            .join_err::<()>("failed to edit workspace")?;
        Ok(())
    }
}

#[cfg(any(test, feature = "integration-tests"))]
impl<R: AppRuntime> Workspace<R> {
    pub fn db(&self) -> &Arc<dyn moss_storage::WorkspaceStorage<R::AsyncContext>> {
        self.storage_service.storage()
    }
}
