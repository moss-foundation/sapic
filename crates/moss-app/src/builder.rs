use anyhow::{Context as _, Result};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{AppRuntime, ServiceMarker, providers::ServiceMap};
use moss_fs::FileSystem;
use std::{
    any::TypeId,
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::AppHandle;
use tokio::sync::RwLock;

use crate::{
    app::{App, AppCommands, AppDefaults, AppPreferences, GlobalsMap},
    command::CommandDecl,
    dirs,
    services::storage_service::StorageService,
};

pub struct AppBuilder<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    app_handle: AppHandle<R::EventLoop>,
    globals: GlobalsMap,
    services: ServiceMap,
    defaults: AppDefaults,
    preferences: AppPreferences,
    commands: AppCommands<R::EventLoop>,
    activity_indicator: ActivityIndicator<R::EventLoop>,
    abs_path: Arc<Path>,
}

impl<R: AppRuntime> AppBuilder<R> {
    pub fn new(
        app_handle: AppHandle<R::EventLoop>,
        activity_indicator: ActivityIndicator<R::EventLoop>,
        defaults: AppDefaults,
        fs: Arc<dyn FileSystem>,
        abs_path: PathBuf,
    ) -> Self {
        Self {
            fs,
            app_handle,
            defaults,
            preferences: AppPreferences {
                theme: RwLock::new(None),
                locale: RwLock::new(None),
            },
            commands: Default::default(),
            globals: Default::default(),
            services: Default::default(),
            activity_indicator,
            abs_path: abs_path.into(),
        }
    }

    // pub fn with_global<T: Send + Sync + 'static>(mut self, global: impl Into<Box<T>>) -> Self {
    //     self.globals.insert(TypeId::of::<T>(), global.into());
    //     self
    // }

    // pub fn with_service<T: ServiceMarker + Send + Sync>(
    //     mut self,
    //     service: impl Into<Arc<T>>,
    // ) -> Self {
    //     self.services.insert(TypeId::of::<T>(), service.into());
    //     self
    // }

    pub fn with_command(mut self, command: CommandDecl<R::EventLoop>) -> Self {
        self.commands.insert(command.name, command.callback);
        self
    }

    pub async fn build(self) -> joinerror::Result<App<R>> {
        for dir in &[dirs::WORKSPACES_DIR, dirs::GLOBALS_DIR] {
            let dir_path = self.abs_path.join(dir);
            if dir_path.exists() {
                continue;
            }

            self.fs
                .create_dir(&dir_path)
                .await
                .context("Failed to create app directories")?;
        }

        let storage_service: Arc<StorageService<R>> =
            StorageService::<R>::new(&self.abs_path.join(dirs::GLOBALS_DIR))
                .map_err(|err| joinerror::Error::new::<()>(err.to_string()))?
                .into();

        Ok(App {
            fs: self.fs,
            app_handle: self.app_handle,
            globals: self.globals,
            commands: self.commands,
            preferences: self.preferences,
            defaults: self.defaults,
            services: self.services.into(),
            tracked_cancellations: Default::default(),
            activity_indicator: self.activity_indicator,
        })
    }
}
