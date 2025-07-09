use derive_more::Deref;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{PublicServiceMarker, providers::ServiceProvider};
use moss_fs::FileSystem;
use moss_text::ReadOnlyStr;
use rustc_hash::FxHashMap;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::RwLock;

use crate::{
    command::CommandCallback,
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
    pub(super) app_handle: AppHandle<R>,
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) commands: AppCommands<R>,
    pub(super) preferences: AppPreferences,
    pub(super) defaults: AppDefaults,
    pub(super) services: ServiceProvider,

    // TODO: This is also might be better to be a service
    pub(super) activity_indicator: ActivityIndicator<R>,
}

impl<R: TauriRuntime> App<R> {
    pub fn handle(&self) -> AppHandle<R> {
        self.app_handle.clone()
    }

    pub fn preferences(&self) -> &AppPreferences {
        &self.preferences
    }

    pub fn defaults(&self) -> &AppDefaults {
        &self.defaults
    }

    pub fn service<T: PublicServiceMarker>(&self) -> &T {
        self.services.get::<T>()
    }

    pub fn command(&self, id: &ReadOnlyStr) -> Option<CommandCallback<R>> {
        self.commands.get(id).map(|cmd| Arc::clone(cmd))
    }
}
