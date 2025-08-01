use anyhow::Result;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{
    AppRuntime, ServiceMarker,
    providers::{ServiceMap, ServiceProvider},
};
use moss_file::json::JsonFileHandle;
use moss_fs::FileSystem;
use std::{any::TypeId, cell::LazyCell, path::Path, sync::Arc};

use crate::{
    Workspace, dirs,
    manifest::{MANIFEST_FILE_NAME, ManifestModel},
    models::types::CreateEnvironmentItemParams,
    services::environment_service::EnvironmentService,
};

struct PredefinedEnvironment {
    name: String,
    order: isize,
    color: Option<String>,
}

const PREDEFINED_ENVIRONMENTS: LazyCell<Vec<PredefinedEnvironment>> = LazyCell::new(|| {
    vec![PredefinedEnvironment {
        name: "Globals".to_string(),
        order: 0,
        color: Some("#3574F0".to_string()),
    }]
});

pub struct LoadWorkspaceParams {
    pub abs_path: Arc<Path>,
}

pub struct CreateWorkspaceParams {
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

    pub async fn initialize(fs: Arc<dyn FileSystem>, params: CreateWorkspaceParams) -> Result<()> {
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

    pub async fn load<R: AppRuntime>(
        self,
        params: LoadWorkspaceParams,
        activity_indicator: ActivityIndicator<R::EventLoop>, // FIXME: will be passed as a service in the future
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

    pub async fn create<R: AppRuntime>(
        self,
        params: CreateWorkspaceParams,
        activity_indicator: ActivityIndicator<R::EventLoop>, // FIXME: will be passed as a service in the future
    ) -> Result<Workspace<R>> {
        debug_assert!(params.abs_path.is_absolute());

        for dir in &[dirs::COLLECTIONS_DIR, dirs::ENVIRONMENTS_DIR] {
            self.fs.create_dir(&params.abs_path.join(dir)).await?;
        }

        let services: ServiceProvider = self.services.into();
        let manifest = JsonFileHandle::create(
            self.fs.clone(),
            &params.abs_path.join(MANIFEST_FILE_NAME),
            ManifestModel { name: params.name },
        )
        .await?;

        let environment_service = services.get::<EnvironmentService<R>>();
        for env in PREDEFINED_ENVIRONMENTS.iter() {
            environment_service
                .create_environment(CreateEnvironmentItemParams {
                    abs_path: params.abs_path.join(dirs::ENVIRONMENTS_DIR),
                    name: env.name.clone(),
                    order: env.order,
                    color: env.color.clone(),
                })
                .await?;
        }

        Ok(Workspace {
            abs_path: params.abs_path,
            activity_indicator,
            manifest,
            services,
        })
    }
}
