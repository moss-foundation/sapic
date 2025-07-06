use anyhow::Result;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{ServiceMarker, providers::ServiceMap};
use moss_file::json::JsonFileHandle;
use moss_fs::FileSystem;
use std::{any::TypeId, path::Path, sync::Arc};
use tauri::Runtime as TauriRuntime;

use crate::{
    Workspace, dirs,
    manifest::{MANIFEST_FILE_NAME, ManifestModel},
};

pub struct WorkspaceLoadParams {
    pub abs_path: Arc<Path>,
}

pub struct WorkspaceCreateParams {
    pub name: String,
    pub abs_path: Arc<Path>,
}

pub struct WorkspaceBuilder {
    fs: Arc<dyn FileSystem>,
    services: ServiceMap,
}

impl WorkspaceBuilder {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self {
            fs,
            services: Default::default(),
        }
    }

    pub async fn initialize(fs: Arc<dyn FileSystem>, params: WorkspaceCreateParams) -> Result<()> {
        debug_assert!(params.abs_path.is_absolute());

        for dir in &[dirs::COLLECTIONS_DIR, dirs::ENVIRONMENTS_DIR] {
            fs.create_dir(&params.abs_path.join(dir)).await?;
        }

        JsonFileHandle::create(
            fs.clone(),
            &params.abs_path.join(MANIFEST_FILE_NAME),
            ManifestModel { name: params.name },
        )
        .await?;

        Ok(())
    }

    pub fn with_service<T: ServiceMarker + Send + Sync>(
        mut self,
        service: impl Into<Arc<T>>,
    ) -> Self {
        self.services.insert(TypeId::of::<T>(), service.into());
        self
    }

    pub async fn load<R: TauriRuntime>(
        self,
        params: WorkspaceLoadParams,
        activity_indicator: ActivityIndicator<R>, // FIXME: will be passed as a service in the future
    ) -> Result<Workspace<R>> {
        debug_assert!(params.abs_path.is_absolute());

        let manifest =
            JsonFileHandle::load(self.fs.clone(), &params.abs_path.join(MANIFEST_FILE_NAME))
                .await?;

        Ok(Workspace {
            abs_path: params.abs_path,
            activity_indicator,
            manifest,
            services: self.services.into(),
        })
    }

    pub async fn create<R: TauriRuntime>(
        self,
        params: WorkspaceCreateParams,
        activity_indicator: ActivityIndicator<R>, // FIXME: will be passed as a service in the future
    ) -> Result<Workspace<R>> {
        debug_assert!(params.abs_path.is_absolute());

        for dir in &[dirs::COLLECTIONS_DIR, dirs::ENVIRONMENTS_DIR] {
            self.fs.create_dir(&params.abs_path.join(dir)).await?;
        }

        let manifest = JsonFileHandle::create(
            self.fs.clone(),
            &params.abs_path.join(MANIFEST_FILE_NAME),
            ManifestModel { name: params.name },
        )
        .await?;

        Ok(Workspace {
            abs_path: params.abs_path,
            activity_indicator,
            manifest,
            services: self.services.into(),
        })
    }
}
