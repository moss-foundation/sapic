use anyhow::{Context as _, Result};
use derive_more::Deref;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::Service;
use moss_fs::FileSystem;
use moss_storage::GlobalStorage;
use moss_text::ReadOnlyStr;
use moss_workspace::context::WorkspaceContext;
use rustc_hash::FxHashMap;
use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::RwLock;

use crate::{
    command::{CommandCallback, CommandDecl},
    dirs,
    models::types::{ColorThemeInfo, LocaleInfo},
    services::workspace_service::{WorkspaceReadGuard, WorkspaceService, WorkspaceWriteGuard},
};

pub struct AppPreferences {
    pub theme: RwLock<Option<ColorThemeInfo>>,
    pub locale: RwLock<Option<LocaleInfo>>,
}

pub struct AppDefaults {
    pub theme: ColorThemeInfo,
    pub locale: LocaleInfo,
}

type AnyService = Arc<dyn Any + Send + Sync>;

#[derive(Default)]
pub struct AppServices(FxHashMap<TypeId, AnyService>);

impl Deref for AppServices {
    type Target = FxHashMap<TypeId, AnyService>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct AppCommands<R: TauriRuntime>(FxHashMap<ReadOnlyStr, CommandCallback<R>>);

impl<R: TauriRuntime> Default for AppCommands<R> {
    fn default() -> Self {
        Self(FxHashMap::default())
    }
}

impl<R: TauriRuntime> Deref for AppCommands<R> {
    type Target = FxHashMap<ReadOnlyStr, CommandCallback<R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: TauriRuntime> DerefMut for AppCommands<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Deref)]
pub struct App<R: TauriRuntime> {
    #[deref]
    pub(crate) app_handle: AppHandle<R>,
    pub(crate) fs: Arc<dyn FileSystem>,
    pub(crate) commands: AppCommands<R>,
    pub(crate) preferences: AppPreferences,
    pub(crate) defaults: AppDefaults,
    pub(crate) services: AppServices,

    // TODO: This is also might be better to be a service
    pub(crate) activity_indicator: ActivityIndicator<R>,
    pub(super) global_storage: Arc<dyn GlobalStorage>,

    // TODO: Not sure this the best place for this, and do we even need it
    pub(crate) abs_path: Arc<Path>,
}

pub struct AppBuilder<R: TauriRuntime> {
    fs: Arc<dyn FileSystem>,
    app_handle: AppHandle<R>,
    services: AppServices,
    defaults: AppDefaults,
    preferences: AppPreferences,
    commands: AppCommands<R>,
    activity_indicator: ActivityIndicator<R>,
    global_storage: Arc<dyn GlobalStorage>,
    abs_path: Arc<Path>,
}

impl<R: TauriRuntime> AppBuilder<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        global_storage: Arc<dyn GlobalStorage>,
        activity_indicator: ActivityIndicator<R>,
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
            services: Default::default(),
            activity_indicator,
            global_storage,
            abs_path: abs_path.into(),
        }
    }

    pub fn with_service<T: Service + Send + Sync>(mut self, service: T) -> Self {
        self.services.insert(TypeId::of::<T>(), Arc::new(service));
        self
    }

    pub fn with_command(mut self, command: CommandDecl<R>) -> Self {
        self.commands.insert(command.name, command.callback);
        self
    }

    pub async fn build(self) -> Result<App<R>> {
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

        Ok(App {
            fs: self.fs,
            app_handle: self.app_handle,
            commands: self.commands,
            preferences: self.preferences,
            defaults: self.defaults,
            services: self.services,
            activity_indicator: self.activity_indicator,
            global_storage: self.global_storage,
            abs_path: self.abs_path,
        })
    }
}
impl<R: TauriRuntime> App<R> {
    pub fn preferences(&self) -> &AppPreferences {
        &self.preferences
    }

    pub fn defaults(&self) -> &AppDefaults {
        &self.defaults
    }

    pub fn service<T: Service>(&self) -> &T {
        let type_id = TypeId::of::<T>();
        let service = self.services.get(&type_id).expect(&format!(
            "Service {} must be registered before it can be used",
            std::any::type_name::<T>()
        ));

        service.downcast_ref::<T>().expect(&format!(
            "Service {} is registered with the wrong type type id",
            std::any::type_name::<T>()
        ))
    }

    pub fn command(&self, id: &ReadOnlyStr) -> Option<CommandCallback<R>> {
        self.commands.get(id).map(|cmd| Arc::clone(cmd))
    }

    pub async fn workspace(&self) -> Option<(WorkspaceReadGuard<'_, R>, WorkspaceContext<R>)> {
        self.service::<WorkspaceService<R>>()
            .workspace_with_context(self.app_handle.clone())
            .await
    }

    pub async fn workspace_mut(&self) -> Option<(WorkspaceWriteGuard<'_, R>, WorkspaceContext<R>)> {
        self.service::<WorkspaceService<R>>()
            .workspace_with_context_mut(self.app_handle.clone())
            .await
    }

    /// Test only utility, not feature-flagged for easier CI setup
    pub fn __storage(&self) -> Arc<dyn GlobalStorage> {
        self.global_storage.clone()
    }
}
