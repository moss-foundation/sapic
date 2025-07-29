use anyhow::Result;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{AppRuntime, PublicServiceMarker, providers::ServiceProvider};
use moss_collection::Collection;
use moss_environment::environment::{AnyEnvironment, Environment};
use moss_file::json::JsonFileHandle;
use moss_fs::FileSystem;
use std::{path::Path, sync::Arc};

use crate::{
    manifest::{MANIFEST_FILE_NAME, ManifestModel},
    services::{AnyCollectionService, AnyEnvironmentService},
};

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

    // type CollectionService: AnyCollectionService<R>;
    // type EnvironmentService: AnyEnvironmentService;
}

pub struct Workspace<R: AppRuntime> {
    pub(super) abs_path: Arc<Path>,
    pub(super) services: ServiceProvider,
    #[allow(dead_code)]
    pub(super) activity_indicator: ActivityIndicator<R::EventLoop>,
    #[allow(dead_code)]
    pub(super) manifest: JsonFileHandle<ManifestModel>,
}

impl<R: AppRuntime> AnyWorkspace<R> for Workspace<R> {
    type Collection = Collection<R>;
    type Environment = Environment<R>;

    // type CollectionService = CollectionService;
    // type EnvironmentService = EnvironmentService;
}

impl<R: AppRuntime> Workspace<R> {
    pub fn service<S: PublicServiceMarker>(&self) -> &S {
        self.services.get::<S>()
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

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    // // Test only utility, not feature-flagged for easier CI setup
    // pub fn __storage(&self) -> Arc<dyn WorkspaceStorage> {
    //     self.storage.clone()
    // }
}
