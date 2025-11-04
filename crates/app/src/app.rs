mod language;
mod profile;
mod theme;

pub mod builder;
pub mod command;
pub mod types;

use derive_more::Deref;
use moss_applib::AppRuntime;
use moss_text::ReadOnlyStr;
use rustc_hash::FxHashMap;
use sapic_window::Window;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tauri::{AppHandle as TauriAppHandle, Runtime as TauriRuntime};
use tokio::sync::RwLock;

use crate::command::CommandCallback;

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
pub struct App<R: AppRuntime> {
    #[deref]
    pub(crate) tauri_handle: TauriAppHandle<R::EventLoop>,
    pub(crate) commands: AppCommands<R::EventLoop>,
    pub(crate) windows: RwLock<FxHashMap<String, Arc<Window<R>>>>,
}

impl<R: AppRuntime> App<R> {
    pub fn handle(&self) -> TauriAppHandle<R::EventLoop> {
        self.tauri_handle.clone()
    }

    pub async fn window(&self, id: &str) -> Option<Arc<Window<R>>> {
        self.windows.read().await.get(id).cloned()
    }

    pub fn command(&self, id: &ReadOnlyStr) -> Option<CommandCallback<R::EventLoop>> {
        self.commands.get(id).map(|cmd| Arc::clone(cmd))
    }
}
