use derive_more::Deref;
use joinerror::{Error, ResultExt};
use moss_applib::{AppRuntime, providers::ServiceProvider};
use moss_fs::{FileSystem, RenameOptions, model_registry::GlobalModelRegistry};
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{
    AnyEnvironment, AnySyncService, ModifyEnvironmentParams,
    services::{sync_service::SyncService, variable_service::VariableService},
};
#[derive(Debug, Deref)]
pub(super) struct EnvironmentPath {
    parent: PathBuf,
    name: String,

    #[deref]
    path: Arc<Path>,
}

impl EnvironmentPath {
    pub fn new(abs_path: Arc<Path>) -> joinerror::Result<Self> {
        debug_assert!(abs_path.is_absolute());

        let parent = abs_path
            .parent()
            .ok_or_else(|| joinerror::Error::new::<()>("environment path must have a parent"))?;

        let name = abs_path
            .file_name()
            .ok_or_else(|| joinerror::Error::new::<()>("environment path must have a name"))?;

        Ok(Self {
            parent: parent.to_path_buf(),
            name: name.to_string_lossy().to_string(),
            path: abs_path,
        })
    }
}

struct EnvironmentState {
    abs_path: EnvironmentPath,
}

pub struct Environment<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    model_registry: Arc<GlobalModelRegistry>,
    state: RwLock<EnvironmentState>,
    services: ServiceProvider,

    _marker: PhantomData<R>,
}

unsafe impl<R: AppRuntime> Send for Environment<R> {}
unsafe impl<R: AppRuntime> Sync for Environment<R> {}

impl<R: AppRuntime> Environment<R> {
    pub(super) fn new(
        abs_path: Arc<Path>,
        fs: Arc<dyn FileSystem>,
        model_registry: Arc<GlobalModelRegistry>,
        services: ServiceProvider,
    ) -> joinerror::Result<Self> {
        let abs_path = EnvironmentPath::new(abs_path)?;

        Ok(Self {
            fs,
            model_registry,
            state: RwLock::new(EnvironmentState { abs_path }),
            services,
            _marker: PhantomData,
        })
    }
}

impl<R: AppRuntime> AnyEnvironment<R> for Environment<R> {
    async fn abs_path(&self) -> Arc<Path> {
        self.state.read().await.abs_path.path.clone()
    }

    async fn name(&self) -> String {
        self.state.read().await.abs_path.name.clone()
    }

    async fn color(&self) -> Option<String> {
        None // TODO: hardcoded for now
    }

    async fn modify(&self, params: ModifyEnvironmentParams) -> joinerror::Result<()> {
        let sync_service = self.services.get::<SyncService>();
        let variable_service = self.services.get::<VariableService<R>>();

        if let Some(new_name) = params.name {
            self.rename(new_name).await?;
        }

        let abs_path = self.state.read().await.abs_path.path.clone();

        variable_service
            .batch_add(&abs_path, params.vars_to_add)
            .await?;
        variable_service
            .batch_remove(&abs_path, params.vars_to_delete)
            .await?;

        // TODO: update metadata(color)

        // TODO: we'll handle file system synchronization in the background a bit later,
        // so we can respond to the frontend faster.

        sync_service.save(&abs_path).await?;

        Ok(())
    }
}

impl<R: AppRuntime> Environment<R> {
    async fn rename(&self, new_name: String) -> joinerror::Result<()> {
        let mut state = self.state.write().await;
        let current_abs_path = state.abs_path.path.clone();
        let new_abs_path: Arc<Path> = state.abs_path.parent.join(new_name).into();
        let environment_path =
            EnvironmentPath::new(new_abs_path.clone()).join_err_with::<()>(|| {
                format!(
                    "failed to create environment path {}",
                    new_abs_path.display()
                )
            })?;

        self.fs
            .rename(
                &current_abs_path,
                &new_abs_path,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .map_err(|err| Error::new::<()>(format!("failed to rename file: {}", err)))?;

        self.model_registry
            .rekey(&current_abs_path, new_abs_path.clone())
            .await;

        state.abs_path = environment_path;

        Ok(())
    }
}
