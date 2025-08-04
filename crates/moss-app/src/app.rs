use derive_more::Deref;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{
    AppRuntime, PublicServiceMarker, context::Canceller, providers::ServiceProvider,
};
use moss_fs::FileSystem;
use moss_text::ReadOnlyStr;
use rustc_hash::FxHashMap;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
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

pub(super) type GlobalsMap = FxHashMap<TypeId, Box<dyn Any + Send + Sync>>;

#[derive(Deref)]
pub struct App<R: AppRuntime> {
    #[deref]
    pub(super) app_handle: AppHandle<R::EventLoop>,
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) globals: GlobalsMap,
    pub(super) commands: AppCommands<R::EventLoop>,
    pub(super) preferences: AppPreferences,
    pub(super) defaults: AppDefaults,
    pub(super) services: ServiceProvider,

    // Store cancellers by the id of API requests
    pub(super) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
    // TODO: This is also might be better to be a service
    pub(super) activity_indicator: ActivityIndicator<R::EventLoop>,
}

impl<R: AppRuntime> App<R> {
    #[track_caller]
    pub fn global<T: Send + Sync + 'static>(&self) -> &T {
        self.globals
            .get(&TypeId::of::<T>())
            .map(|any_global| any_global.downcast_ref::<T>().unwrap())
            .unwrap_or_else(|| panic!("no state of type {} exists", std::any::type_name::<T>()))
    }

    pub fn handle(&self) -> AppHandle<R::EventLoop> {
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

    pub fn command(&self, id: &ReadOnlyStr) -> Option<CommandCallback<R::EventLoop>> {
        self.commands.get(id).map(|cmd| Arc::clone(cmd))
    }

    pub async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.insert(request_id.to_string(), canceller);
    }

    pub async fn release_cancellation(&self, request_id: &str) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.remove(request_id);
    }

    #[cfg(feature = "integration-tests")]
    pub fn cancellation_map(&self) -> Arc<RwLock<HashMap<String, Canceller>>> {
        self.tracked_cancellations.clone()
    }
}
