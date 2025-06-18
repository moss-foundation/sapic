use moss_applib::Service;
use moss_fs::FileSystem;
use moss_text::ReadOnlyStr;
use moss_workbench::workbench::Workbench;
use rustc_hash::FxHashMap;
use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::RwLock;

use crate::{
    command::{CommandCallback, CommandDecl},
    models::types::{ColorThemeInfo, LocaleInfo},
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

pub struct App<R: TauriRuntime> {
    pub(crate) fs: Arc<dyn FileSystem>,
    pub(crate) app_handle: AppHandle<R>,
    pub(crate) workbench: Workbench<R>,
    pub(crate) commands: AppCommands<R>,
    pub(crate) preferences: AppPreferences,
    pub(crate) defaults: AppDefaults,
    pub(crate) services: AppServices,
}

impl<R: TauriRuntime> Deref for App<R> {
    type Target = AppHandle<R>;

    fn deref(&self) -> &Self::Target {
        &self.app_handle
    }
}

pub struct AppBuilder<R: TauriRuntime> {
    fs: Arc<dyn FileSystem>,
    app_handle: AppHandle<R>,
    workbench: Workbench<R>,
    services: AppServices,
    defaults: AppDefaults,
    preferences: AppPreferences,
    commands: AppCommands<R>,
}

impl<R: TauriRuntime> AppBuilder<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        workbench: Workbench<R>,
        defaults: AppDefaults,
        fs: Arc<dyn FileSystem>,
    ) -> Self {
        Self {
            fs,
            app_handle,
            workbench,
            defaults,
            preferences: AppPreferences {
                theme: RwLock::new(None),
                locale: RwLock::new(None),
            },
            commands: Default::default(),
            services: Default::default(),
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

    pub fn build(self) -> App<R> {
        App {
            fs: self.fs,
            app_handle: self.app_handle,
            workbench: self.workbench,
            commands: self.commands,
            preferences: self.preferences,
            defaults: self.defaults,
            services: self.services,
        }
    }
}
impl<R: TauriRuntime> App<R> {
    pub fn workbench(&self) -> &Workbench<R> {
        &self.workbench
    }

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
}
